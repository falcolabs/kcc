/// An asset.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
pub enum Asset {
    Costume(Costume),
    Sound(Sound),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Costume {
    pub name: String,
    pub data_format: String,
    pub asset_id: String,
    pub md5ext: String,
    
    pub rotation_center_x: f64,
    pub rotation_center_y: f64,
    pub bitmap_resolution: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Sound {
    pub name: String,
    pub data_format: String,
    pub asset_id: String,
    pub md5ext: String,

    pub rate: i32,
    pub sample_count: i32,
}
