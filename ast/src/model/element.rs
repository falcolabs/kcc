use std::collections::HashMap;

use crate::errors::ScratchError;

use super::BlockType;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
#[repr(i32)]
pub enum ShadowType {
    Literal = 1,
    FilledEmptySlot = 2,
    OverrideValue = 3,
}

#[derive(Clone)]
/// A reporter-like block, in which data could be entered
/// or selected from a list, and could be replaced by a normal reporter.
/// An example could be the input text box of the say block.
pub struct Shadow {
    /// The input type of the input. 1 is a shadow (non-draggable values),
    /// 2 is a bare value, and 3 is a shadow but overwritten by a reporter.
    pub shadow_type: ShadowType,
    /// The value of the input
    pub value: Option<ShadowValue>,
    /// The original value if it was obscured (`input_type` is 3)
    pub overridden_value: Option<ShadowValue>,
}

impl PartialEq for Shadow {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl std::fmt::Debug for Shadow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:#?}", self.value))
    }
}

#[derive(Debug, Clone, PartialEq)]
/// The value of a field, present in a reporter.
pub struct Field {
    /// The value of the field.
    pub value: String,
    /// The ID of the field's value.
    pub value_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
/// The Define block, its inner part, a Custom block invocation, or Stop block.
pub enum Mutation {
    ProcedureCall(ProcedureCall),
    ProcedurePrototype(ProcedurePrototype),
    // Really?
    ControlStop(ControlStopMutation),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
/// The invocation of a procedure, containing info about the arguments
/// and the procedure's name.
pub struct ProcedureCall {
    /// Often an empty list.
    pub children: Vec<String>,
    /// Always 'mutation'
    #[cfg_attr(feature = "serde", serde(default = "defmutation"))]
    pub tag_name: String,
    /// The name of the custom block, including inputs: %s for string/number inputs and %b for boolean inputs.
    pub proccode: String,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "argumentids", with = "serde_nested_json")
    )]
    pub arguments_ids: Vec<String>,
    /// Whether to run the block without screen refresh or not.
    #[cfg_attr(feature = "serde", serde(with = "serde_nested_json"))]
    pub warp: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
/// The prototype of a procedure, specifying its name and argument list.
pub struct ProcedurePrototype {
    /// Often an empty list.
    pub children: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default = "defmutation"))]
    /// Always 'mutation'
    pub tag_name: String,
    /// The name of the custom block, including inputs: %s for string/number inputs and %b for boolean inputs.
    pub proccode: String,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "argumentids", with = "serde_nested_json")
    )]
    pub arguments_ids: Vec<String>,
    #[cfg_attr(feature = "serde", serde(with = "serde_nested_json"))]
    pub warp: bool,

    #[cfg_attr(
        feature = "serde",
        serde(rename = "argumentnames", with = "serde_nested_json")
    )]
    /// A JSON string that contains an array of the names of the arguments.
    pub argument_names: Vec<String>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "argumentdefaults", with = "serde_nested_json")
    )]
    /// A JSON string that contains an array of the defaults of the arguments;
    /// for string/number arguments, this is an empty string, and for boolean arguments it is false.
    pub argument_defaults: Vec<PrimitiveValue>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
/// The prototype of a procedure, specifying its name and argument list.
pub struct ControlStopMutation {
    /// Often an empty list.
    pub children: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default = "defmutation"))]
    /// Always 'mutation'
    pub tag_name: String,

    #[cfg_attr(feature = "serde", serde(rename = "hasnext"))]
    pub has_next: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
/// A generic Scratch block.
pub struct Block {
    #[cfg_attr(feature = "serde", serde(skip, default = "defna"))]
    pub obj_id: String,
    #[cfg_attr(feature = "serde", serde(rename = "opcode"))]
    pub block_type: BlockType,
    #[cfg_attr(feature = "serde", serde(rename = "next"))]
    pub next_id: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "parent"))]
    pub parent_id: Option<String>,
    pub inputs: HashMap<String, Shadow>,
    pub fields: HashMap<String, Field>,
    pub shadow: bool,
    pub top_level: bool,
    pub x: Option<f64>,
    pub y: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default = "defnone"))]
    pub comment_id: Option<String>,
    #[cfg_attr(feature = "serde", serde(default = "defnone"))]
    pub mutation: Option<Mutation>,
    // TODO
    // @property
    // def parent(self) -> "Block" | None:
    //     return get_block(self.parent_id)

    // @property
    // def next(self) -> "Block" | None:
    //     return get_block(self.next_id)

    // @property
    // def comment(self) -> "Comment" | None:
    //     return get_comment(self.comment_id)
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
pub enum RichValue {
    Boolean(bool),
    Number(f64),
    PositiveNumber(f64),
    Integer(i64),
    PositiveInteger(u32),
    Angle(f64),
    Color(String),
    Broadcast(String),
    String(String),
}

impl RichValue {
    pub fn success() -> Self {
        Self::Boolean(true)
    }

    pub fn get_array_representation_number(&self) -> i32 {
        match self {
            RichValue::Boolean(_) => 10,
            RichValue::Number(_) => 4,
            RichValue::PositiveNumber(_) => 5,
            RichValue::PositiveInteger(_) => 6,
            RichValue::Integer(_) => 7,
            RichValue::Angle(_) => 8,
            RichValue::Color(_) => 9,
            RichValue::String(_) => 10,
            RichValue::Broadcast(_) => 11,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
pub enum PrimitiveValue {
    Number(f64),
    Integer(i64),
    String(String),
}

impl From<PrimitiveValue> for String {
    fn from(value: PrimitiveValue) -> Self {
        match value {
            PrimitiveValue::String(s) => s,
            PrimitiveValue::Integer(i) => i.to_string(),
            PrimitiveValue::Number(i) => {
                if i.is_nan() {
                    return String::from("NaN");
                }
                if i.is_infinite() {
                    if i.is_sign_negative() {
                        return String::from("-Infinity");
                    }
                    return String::from("Infinity");
                }

                i.to_string()
            }
        }
    }
}

impl TryFrom<PrimitiveValue> for f64 {
    type Error = ScratchError;

    fn try_from(value: PrimitiveValue) -> Result<Self, Self::Error> {
        match value {
            PrimitiveValue::String(s) => s.parse().map_err(|e| {
                ScratchError::type_error(
                    format!("Cannot convert {:#?} to an f64: {e}", s),
                    format!("converting {:#?} to an f64: {e}", s),
                )
            }),
            PrimitiveValue::Integer(i) => Ok(i as f64),
            PrimitiveValue::Number(i) => Ok(i),
        }
    }
}

impl TryFrom<RichValue> for f64 {
    type Error = ScratchError;

    fn try_from(value: RichValue) -> Result<Self, Self::Error> {
        let pv: PrimitiveValue = value.into();
        pv.try_into()
    }
}

impl From<f64> for PrimitiveValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<f64> for RichValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i64> for PrimitiveValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<i64> for RichValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<String> for PrimitiveValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<String> for RichValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for PrimitiveValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<&str> for RichValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl TryInto<bool> for PrimitiveValue {
    type Error = String;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            PrimitiveValue::String(s) => {
                if s == "true" {
                    Ok(true)
                } else if s == "false" {
                    Ok(false)
                } else {
                    Err(format!("Cannot convert {s:#?} to a boolean"))
                }
            }
            PrimitiveValue::Integer(i) => Err(format!("Cannot convert {i:#?} to a boolean")),
            PrimitiveValue::Number(i) => Err(format!("Cannot convert {i:#?} to a boolean")),
        }
    }
}

impl From<PrimitiveValue> for RichValue {
    fn from(value: PrimitiveValue) -> RichValue {
        match value {
            PrimitiveValue::Number(n) => RichValue::Number(n),
            PrimitiveValue::Integer(n) => RichValue::Integer(n),
            PrimitiveValue::String(s) => {
                if s.len() == 7 && s.starts_with("#") {
                    return RichValue::Color(s);
                }
                RichValue::String(s)
            }
        }
    }
}

impl From<RichValue> for PrimitiveValue {
    fn from(value: RichValue) -> Self {
        match value {
            RichValue::Angle(n) => PrimitiveValue::Number(n),
            RichValue::Boolean(n) => {
                if n {
                    return PrimitiveValue::String("true".to_string());
                }
                PrimitiveValue::String("false".to_string())
            }
            RichValue::Number(n) => PrimitiveValue::Number(n),
            RichValue::PositiveInteger(n) => PrimitiveValue::Integer(n.into()),
            RichValue::Integer(n) => PrimitiveValue::Integer(n),
            RichValue::PositiveNumber(n) => PrimitiveValue::Number(n),
            RichValue::String(n) => PrimitiveValue::String(n),
            RichValue::Color(n) => PrimitiveValue::String(n),
            RichValue::Broadcast(n) => PrimitiveValue::String(n),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValuePointer {
    Variable { name: String, id: String },
    List { name: String, id: String },
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct VariableRef {
//     pub name: String,
//     pub variable_id: String,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct ListRef {
//     pub name: String,
//     pub list_id: String,
// }

#[derive(Debug, Clone, PartialEq)]
pub struct BlockRef {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Evaluable {
    Bare(RichValue),
    Block(BlockRef),
    Shadow(Shadow),
    Pointer(ValuePointer),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShadowValue {
    Bare(RichValue),
    Block(BlockRef),
    Pointer(ValuePointer),
}

impl From<ShadowValue> for Evaluable {
    fn from(value: ShadowValue) -> Self {
        match value {
            ShadowValue::Bare(bv) => Evaluable::Bare(bv),
            ShadowValue::Block(b) => Evaluable::Block(b),
            ShadowValue::Pointer(p) => Evaluable::Pointer(p),
        }
    }
}

impl RichValue {
    // TODO - make this call eval
    pub fn as_str(&self) -> Result<String, &str> {
        if let Self::String(r) = self.clone() {
            return Ok(r);
        }
        Err("PartialValue type mismatch.")
    }

    pub fn as_number(&self) -> Result<f64, &str> {
        if let Self::Number(r) = self.clone() {
            return Ok(r);
        }
        if let Self::String(r) = self.clone()
            && let Ok(f) = r.parse::<f64>()
        {
            return Ok(f);
        }
        Err("PartialValue type mismatch.")
    }

    pub fn as_bool(&self) -> Result<bool, &str> {
        if let Self::Boolean(r) = self.clone() {
            return Ok(r);
        }
        if let Self::Number(r) = self.clone() {
            return Ok(r == 1.0);
        }
        Err("PartialValue type mismatch.")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Comment {
    /// The ID of this comment
    #[serde(skip)]
    pub obj_id: String,
    /// The block this comment is attached to.
    pub block_id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub minimized: bool,
    pub text: String,
    // TODO
    // @property
    // def attached_to(self) -> Block:
    //     return get_block(self.block_id)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub value: PrimitiveValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub name: String,
    pub value: Vec<PrimitiveValue>,
}

fn defnone<T>() -> Option<T> {
    Option::None
}
fn defmutation() -> String {
    String::from("mutation")
}

fn defna() -> String {
    String::from("N/A")
}
