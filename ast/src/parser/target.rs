use std::collections::HashMap;

use serde::{de::value::MapDeserializer, Serialize};
use serde::de::Visitor;
use serde::Deserialize;
use serde_json::Value;

use crate::model::target::{Sprite, Stage, Target};
use crate::model::RotationStyle;

impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_map(TargetVisitor {})
    }
}

struct TargetVisitor;

impl<'de> Visitor<'de> for TargetVisitor {
    type Value = Target;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON object property representing a target")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>         
    where
        A: serde::de::MapAccess<'de>, 
        {
        let mut h: HashMap<String, Value> = HashMap::new();
        while let Some((prop, val)) = map.next_entry::<String, serde_json::Value>()? {
            h.insert(prop, val);
        }

        if let Some(v) = h.get("isStage") {
            if v.as_bool().unwrap() {
                return Ok(Target::Stage(Stage::deserialize(MapDeserializer::new(h.into_iter())).map_err(|e| -> A::Error {serde::de::Error::custom(e.to_string())})?));
            } else {
                return Ok(Target::Sprite(Sprite::deserialize(MapDeserializer::new(h.into_iter())).map_err(|e| -> A::Error {serde::de::Error::custom(e.to_string())})?));
            }
        }

        Err(serde::de::Error::missing_field("isStage"))
    }
}

struct RotationStyleVisitor;

impl Serialize for RotationStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match self {
            RotationStyle::AllAround => serializer.serialize_str("all around"),
            RotationStyle::LeftRight => serializer.serialize_str("left-right"),
            RotationStyle::DontRotate => serializer.serialize_str("don't rotate"),
        }
    }
}

impl<'de> Deserialize<'de> for RotationStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_str(RotationStyleVisitor {})
    }
}

impl<'de> Visitor<'de> for RotationStyleVisitor {
    type Value = RotationStyle;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("\"all around\", \"left-right\", or \"don't rotate\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v {
            "all around" => Ok(RotationStyle::AllAround),
            "left-right" => Ok(RotationStyle::LeftRight),
            "don't rotate" => Ok(RotationStyle::DontRotate),
            _ => Err(serde::de::Error::custom("invalid rotation style"))
        }
    }
}




