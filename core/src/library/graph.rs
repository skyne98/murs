use std::{collections::HashMap, path::PathBuf};

use super::{link::LibraryLink, Library};
use anyhow::{anyhow, Result};
use pathfinding::directed::topological_sort::topological_sort;

pub struct LibraryResolutionGraph {
    pub roots: Vec<String>,
    pub nodes: HashMap<String, LibraryResolutionGraphNode>,
    pub children: HashMap<String, Vec<String>>,
}
impl LibraryResolutionGraph {
    pub fn sorted_nodes(&self) -> Result<Vec<&LibraryResolutionGraphNode>> {
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
                .collect::<Vec<&LibraryResolutionGraphNode>>();
            sorted_nodes.reverse();
            Ok(sorted_nodes)
        }
    }
}
pub struct LibraryResolutionGraphNode {
    pub link: LibraryLink,
    pub path: PathBuf,
    pub library: Library,
}
