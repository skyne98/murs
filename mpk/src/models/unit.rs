use anyhow::{anyhow, Context, Result};
use minimad::{parse_text, CompositeStyle, Line};

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    pub name: String,
    pub title: String,
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
