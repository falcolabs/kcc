pub mod comment;
pub mod field;
pub mod input;
pub mod list;
pub mod target;
pub mod value;
pub mod variable;

use std::fs::File;
use std::path::PathBuf;

use crate::model::project::Project;

pub fn load_from_directory(path: &std::path::Path) -> Result<Project, String> {
    let f = File::open({
        let mut pf = PathBuf::new();
        pf.push(path.as_os_str().to_str().unwrap());
        pf.push("project.json");
        pf
    })
    .map_err(|err| -> String { err.to_string() })?;

    serde_json::from_reader(f).map_err(|err: serde_json::Error| -> String { err.to_string() })
}
