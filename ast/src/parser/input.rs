use crate::model::ShadowType;
use crate::model::element::Shadow;
use serde::de::Visitor;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};

struct InputVisitor;

impl Serialize for Shadow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        seq.serialize_element(&self.shadow_type)?;
        seq.serialize_element(&self.value)?;
        if let Some(ov) = &self.overridden_value {
            seq.serialize_element(ov)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Shadow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(InputVisitor::new())
    }
}

impl InputVisitor {
    pub fn new() -> InputVisitor {
        InputVisitor {}
    }
}

impl<'de> Visitor<'de> for InputVisitor {
    type Value = Shadow;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON object property representing a scratch Input")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let shadow_type: ShadowType = seq.next_element()?.expect("Invalid shadow type");
        Ok(match shadow_type {
            ShadowType::Literal => {
                Shadow {
                    shadow_type,
                    value: seq.next_element()?.expect("Malformed shadow array"),
                    overridden_value: None,
                }
            }
            ShadowType::FilledEmptySlot => todo!(),
            ShadowType::OverrideValue => {
                Shadow {
                    shadow_type,
                    value: seq.next_element()?.expect("Malformed shadow array"),
                    overridden_value: seq.next_element()?.expect("Malformed shadow array"),
                }
            }
        })
    }
}
