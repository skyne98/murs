use std::path::Path;

use anyhow::{anyhow, Context, Result};
use minimad::{parse_text, CompositeStyle, Line};
use tokio_stream::StreamExt;

use crate::utils::fs::read_file_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    /// Unique name of the unit used to refer to it from other modules and units via `<module-name>/<unit-name>` or `<unit-name>`.
    pub name: String,
    /// Alternative non-unique name used for humans. System usually takes the first header of the content as a title.
    pub title: String,
    /// Actual contents of the unit in Markdown.
    pub contents: String,
}

impl Unit {
    pub async fn from_str<S>(name: S, contents: S) -> Result<Self>
    where
        S: Into<String>,
    {
        let name = name.into();
        let contents = contents.into();
        let contents_parsed = parse_text(&contents);
        let first_line = contents_parsed
            .lines
            .first()
            .context("Unit's contents are empty")?;

        if let Line::Normal(first_line_normal) = first_line {
            if let CompositeStyle::Header(_) = first_line_normal.style {
                let title = first_line_normal.compounds.first().unwrap().to_string();
                return Ok(Unit {
                    name: name,
                    title: title,
                    contents: contents,
                });
            }
        }

        Err(anyhow!("Unit's markdown must start with a header"))
    }

    pub async fn from_path<P: AsRef<Path>>(root: P) -> Result<Vec<Unit>> {
        let root = root.as_ref();
        let read_dir = tokio::fs::read_dir(root).await?;
        let mut read_dir_stream = tokio_stream::wrappers::ReadDirStream::new(read_dir);
        let mut files = vec![];
        while let Some(entry) = read_dir_stream.next().await {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type().await {
                    if file_type.is_file() {
                        let unit_str = read_file_str(entry.path()).await?;
                        let name = entry
                            .path()
                            .file_stem()
                            .context("Cannot get file name")?
                            .to_str()
                            .context("Cannot convert OsStr to &str")?
                            .to_string();
                        let unit = Unit::from_str(name, unit_str).await?;
                        files.push(unit);
                    }
                }
            }
        }

        Ok(files)
    }
}
