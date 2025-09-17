use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::model::assets::{Costume, Sound};
use crate::model::element::{Block, Comment, List};
use crate::model::Variable;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Target {
    Sprite(Sprite),
    Stage(Stage),
}

impl Target {
    pub fn blocks(&self) -> HashMap<String, Block> {
        match self {
            Self::Sprite(s) => {
                s.blocks.clone()
            },
            Self::Stage(s) => {
                s.blocks.clone()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RotationStyle {
    AllAround,
    LeftRight,
    DontRotate,
}

impl RotationStyle {
    pub fn value(&self) -> &'static str {
        match self {
            Self::AllAround => "all around",
            Self::LeftRight => "left-right",
            Self::DontRotate => "don't rotate",
        }   
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct VariableList(pub Vec<Variable>);

#[derive(Clone, PartialEq, Debug)]
pub struct ListSequence(pub Vec<List>);

#[derive(Clone, PartialEq, Debug)]
pub struct CommentList(pub Vec<Comment>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    pub name: String,
    pub blocks: HashMap<String, Block>,
    pub current_costume: i32,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub layer_order: i32,
    pub is_stage: bool,
    pub volume: i32,
    pub broadcasts: HashMap<String, String>,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub comments: CommentList, 

    pub visible: bool,
    pub x: f64,
    pub y: f64,
    pub size: i32,
    pub direction: i32,
    pub draggable: bool,
    pub rotation_style: RotationStyle,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    pub name: String,
    pub blocks: HashMap<String, Block>,
    pub current_costume: i32,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub layer_order: i32,
    pub is_stage: bool,
    pub volume: i32,
    pub broadcasts: HashMap<String, String>,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub comments: CommentList, 

    pub video_transparency: i32,
    pub video_state: String,
    #[serde(default = "default_tempo")]
    pub tempo: i32,
    #[serde(default = "default_none")]
    pub text_to_speech_language: Option<String>,
}

fn default_tempo() -> i32 {60}
fn default_none() -> Option<String> {Option::None}