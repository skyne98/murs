use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    /// Unique name of the unit used to refer to it from other modules and units via "\<module-name\>/\<unit-name\>" or "self/\<unit-name\>"
    name: String,
    /// Virtual path from the root of the module used to render navigation in the future
    path: PathBuf,
}
