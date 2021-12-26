use std::collections::HashMap;

use crate::{cache::Cache, module::model::ModuleModel};

use super::model::LibraryModel;
use anyhow::{anyhow, Result};
use pathfinding::directed::topological_sort::topological_sort;
use semver::VersionReq;

pub const DEFAULT_ROOT: &str = "https://github.com/skyne98/murs-library";

pub struct LibraryResolutionGraph {
    pub roots: Vec<String>,
    pub nodes: HashMap<String, LibraryModel>,
    pub children: HashMap<String, Vec<String>>,
}
impl LibraryResolutionGraph {
    pub async fn from_roots(roots: Vec<LibraryModel>) -> Result<Self> {
        // Beginning from roots, recursively check out libraries to the cache and build the graph
        let cache = Cache::new().await?;
        let mut nodes = HashMap::new();
        let mut children = HashMap::new();

        let mut to_check = roots.clone();
        while let Some(library) = to_check.pop() {
            let name = library.link.to_string();
            nodes.insert(name.clone(), library.clone());
            let links = library.library.links();
            for child in links {
                let child_name = child.to_string();
                children.insert(name.clone(), vec![child_name.clone()]);

                if nodes.contains_key(&child_name) == false {
                    let child_library = cache.library_link(&child).await?;
                    to_check.push(child_library);
                }
            }
        }

        // Check for cycles
        let mut visited = HashMap::new();
        for (name, _) in nodes.iter() {
            if visited.contains_key(name) {
                continue;
            }
            let mut stack = vec![name.clone()];
            while let Some(node) = stack.pop() {
                if visited.contains_key(&node) {
                    continue;
                }
                visited.insert(node.clone(), true);
                if let Some(children) = children.get(&node) {
                    for child in children {
                        stack.push(child.clone());
                    }
                }
            }
        }
        if visited.len() != nodes.len() {
            let mut cycle = vec![];
            for (name, _) in visited.iter() {
                cycle.push(name.clone());
            }
            return Err(anyhow!("Library graph has a cycle: {:?}", cycle));
        }

        Ok(LibraryResolutionGraph {
            roots: roots.iter().map(|l| l.link.to_string()).collect(),
            nodes,
            children,
        })
    }
    pub fn sorted_nodes(&self) -> Result<Vec<&LibraryModel>> {
        // Topologically sort the graph nodes from roots to leaves
        let sorted_nodes = topological_sort(&self.roots, |node_id| {
            self.children
                .get(node_id)
                .map_or(vec![], |children| children.iter().cloned().collect())
        });

        if let Err(err) = sorted_nodes {
            let node = self.nodes.get(&err).unwrap();
            return Err(anyhow!("Library {} creates a cycle", node.link.to_string()));
        } else {
            let sorted_nodes = sorted_nodes.unwrap();
            let mut sorted_nodes = sorted_nodes
                .iter()
                .map(|node_id| self.nodes.get(node_id).unwrap())
                .collect::<Vec<&LibraryModel>>();
            sorted_nodes.reverse();
            Ok(sorted_nodes)
        }
    }
    pub async fn lookup_module(&self, name: &str) -> Result<Vec<ModuleModel>> {
        let sorted_nodes = self.sorted_nodes()?;
        let mut modules = vec![];
        for node in sorted_nodes.iter() {
            let modules_from_library = node.lookup_module(name).await?;
            modules.extend(modules_from_library);
        }

        Ok(modules)
    }
    pub async fn best_module(
        &self,
        name: &str,
        version_req: &VersionReq,
    ) -> Result<Option<ModuleModel>> {
        let modules = self.lookup_module(name).await?;
        let mut best_module = None;
        for module in modules.iter() {
            if version_req.matches(&module.module.version) {
                if best_module.is_none() {
                    best_module = Some(module.clone());
                } else {
                    let best_module_unwrapped = best_module.as_ref().unwrap();
                    if best_module_unwrapped.module.version < module.module.version {
                        best_module = Some(module.clone());
                    }
                }
            }
        }
        Ok(best_module)
    }
}
