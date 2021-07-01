use anyhow::{anyhow, Context, Result};
use minimad::{parse_text, CompositeStyle, Line};

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
}
