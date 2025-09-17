use crate::model::target::Target;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Metadata {
    /// The version of ScratchVM that the project was created with.
    pub vm: String,
    /// Always 3.0.0 for .sb3 file format.
    pub semver: String,
    /// The user agent of the last person to edit the project.
    pub agent: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Project {
    pub meta: Metadata,
    pub targets: Vec<Target>,
}
