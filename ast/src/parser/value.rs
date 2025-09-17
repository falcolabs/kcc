use log::{info};
use serde::Deserialize;
use serde::ser::SerializeSeq;
use serde::{Serialize, de::Visitor};

use crate::model::{BlockRef, Evaluable, RichValue, ShadowType, ShadowValue, ValuePointer};

struct BlockRefVisitor;

impl Serialize for BlockRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.id)
    }
}

impl<'de> Deserialize<'de> for BlockRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(BlockRefVisitor {})
    }
}

impl<'de> Visitor<'de> for BlockRefVisitor {
    type Value = BlockRef;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockRef { id: v })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockRef { id: v.to_string() })
    }
}

impl Serialize for ValuePointer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3)).unwrap();
        match self {
            ValuePointer::Variable { name, id } => {
                seq.serialize_element(&12)?;
                seq.serialize_element(name)?;
                seq.serialize_element(id)?;
                seq.end()
            }
            ValuePointer::List { name, id } => {
                seq.serialize_element(&13)?;
                seq.serialize_element(name)?;
                seq.serialize_element(id)?;
                seq.end()
            }
        }
    }
}

impl Serialize for Evaluable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Evaluable::Bare(rv) => {
                let mut seq = serializer.serialize_seq(Some(2)).unwrap();
                seq.serialize_element(&rv.get_array_representation_number())?;
                seq.serialize_element(rv)?;
                seq.end()
            }
            Evaluable::Block(b) => serializer.serialize_str(&b.id),
            Evaluable::Pointer(p) => p.serialize(serializer),
            Evaluable::Shadow(s) => match s.shadow_type {
                ShadowType::Literal => {
                    let mut seq = serializer.serialize_seq(Some(3)).unwrap();
                    seq.serialize_element(&s.shadow_type)?;
                    seq.serialize_element(&s.value)?;
                    seq.end()
                }
                ShadowType::OverrideValue => {
                    let mut seq = serializer.serialize_seq(Some(3)).unwrap();
                    seq.serialize_element(&s.shadow_type)?;
                    seq.serialize_element(&s.value)?;
                    seq.serialize_element(&s.overridden_value.as_ref().unwrap())?;
                    seq.end()
                }
                ShadowType::FilledEmptySlot => {
                    let mut seq = serializer.serialize_seq(Some(2)).unwrap();
                    seq.serialize_element(&s.shadow_type)?;
                    seq.serialize_element(&s.value)?;
                    seq.end()
                },
            },
        }
    }
}

struct ValueVisitor;

impl<'de> Deserialize<'de> for Evaluable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor {})
    }
}

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Evaluable;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid Scratch value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Evaluable::Bare(RichValue::Boolean(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Evaluable::Bare(RichValue::Number(v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Evaluable::Block(BlockRef { id: v }))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Evaluable::Block(BlockRef { id: v.to_string() }))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let it: i32 = seq.next_element().unwrap().unwrap();
        match it {
            4 => Ok(Evaluable::Bare(RichValue::Number(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into a number; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0.0 to maintain compatibility with llk/scratch-vm.", e); 0.0}),
            ))),
            5 => Ok(Evaluable::Bare(RichValue::PositiveNumber(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into a positive number; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0.0 to maintain compatibility with llk/scratch-vm.", e); 0.0}),
            ))),
            6 => Ok(Evaluable::Bare(RichValue::PositiveInteger(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into an integer; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0 to maintain compatibility with llk/scratch-vm.", e); 0}),
            ))),
            7 => Ok(Evaluable::Bare(RichValue::Integer(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into a positive integer; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0 to maintain compatibility with llk/scratch-vm.", e); 0}),
            ))),
            8 => Ok(Evaluable::Bare(RichValue::Angle(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into a positive integer; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0.0 rad to maintain compatibility with llk/scratch-vm.", e); 0.0}),
            ))),
            9 => Ok(Evaluable::Bare(RichValue::Color(
                seq.next_element().unwrap().expect("Malformed project file"),
            ))),
            10 => Ok(Evaluable::Bare(RichValue::String(
                seq.next_element().unwrap().expect("Malformed project file"),
            ))),
            11 => Ok(Evaluable::Bare(RichValue::Broadcast(
                seq.next_element().unwrap().expect("Malformed project file"),
            ))),
            12 => Ok(Evaluable::Pointer(ValuePointer::Variable {
                name: seq.next_element().unwrap().unwrap(),
                id: seq.next_element().unwrap().unwrap(),
            })),
            13 => Ok(Evaluable::Pointer(ValuePointer::List {
                name: seq.next_element().unwrap().unwrap(),
                id: seq.next_element().unwrap().unwrap(),
            })),
            _ => Err(serde::de::Error::custom(
                "cannot parse values/enums with indicator other than positive integers from 4-13 (inclusive)",
            )),
        }
    }
}

struct ShadowValueVisitor;

impl Serialize for ShadowValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ShadowValue::Bare(rv) => {
                let mut seq = serializer.serialize_seq(Some(2)).unwrap();
                seq.serialize_element(&rv.get_array_representation_number())?;
                seq.serialize_element(rv)?;
                seq.end()
            }
            ShadowValue::Block(b) => serializer.serialize_str(&b.id),
            ShadowValue::Pointer(p) => p.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ShadowValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ShadowValueVisitor {})
    }
}

impl<'de> Visitor<'de> for ShadowValueVisitor {
    type Value = ShadowValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid Scratch value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ShadowValue::Bare(RichValue::Boolean(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ShadowValue::Bare(RichValue::Number(v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ShadowValue::Block(BlockRef { id: v }))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ShadowValue::Block(BlockRef { id: v.to_string() }))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let it: i32 = seq.next_element().unwrap().unwrap();
        match it {
            4 => Ok(ShadowValue::Bare(RichValue::Number(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into a number; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0.0 to maintain compatibility with llk/scratch-vm.", e); 0.0}),
            ))),
            5 => Ok(ShadowValue::Bare(RichValue::PositiveNumber(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into a positive number; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0.0 to maintain compatibility with llk/scratch-vm.", e); 0.0}),
            ))),
            6 => Ok(ShadowValue::Bare(RichValue::PositiveInteger(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into an integer; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0 to maintain compatibility with llk/scratch-vm.", e); 0}),
            ))),
            7 => Ok(ShadowValue::Bare(RichValue::Integer(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into a positive integer; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0 to maintain compatibility with llk/scratch-vm.", e); 0}),
            ))),
            8 => Ok(ShadowValue::Bare(RichValue::Angle(
                seq.next_element::<String>()
                    .unwrap()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|e| {info!("{}; the value of the input cannot be parsed into a positive integer; however it may still work because it has been overriden by a reporter (rounded edges) block. It will be parsed as 0.0 rad to maintain compatibility with llk/scratch-vm.", e); 0.0}),
            ))),
            9 => Ok(ShadowValue::Bare(RichValue::Color(
                seq.next_element().unwrap().expect("Malformed project file"),
            ))),
            10 => Ok(ShadowValue::Bare(RichValue::String(
                seq.next_element().unwrap().expect("Malformed project file"),
            ))),
            11 => Ok(ShadowValue::Bare(RichValue::Broadcast(
                seq.next_element().unwrap().expect("Malformed project file"),
            ))),
            12 => Ok(ShadowValue::Pointer(ValuePointer::Variable {
                name: seq.next_element().unwrap().unwrap(),
                id: seq.next_element().unwrap().unwrap(),
            })),
            13 => Ok(ShadowValue::Pointer(ValuePointer::List {
                name: seq.next_element().unwrap().unwrap(),
                id: seq.next_element().unwrap().unwrap(),
            })),
            _ => Err(serde::de::Error::custom(
                "cannot parse values/enums with indicator other than positive integers from 4-13 (inclusive)",
            )),
        }
    }
}
