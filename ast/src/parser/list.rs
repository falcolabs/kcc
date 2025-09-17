use crate::model::PrimitiveValue;
use crate::model::element::List;
use serde::de::Visitor;
use serde::ser::{SerializeSeq};
use serde::{Deserialize, Serialize};

struct ListVisitor;

impl Serialize for List {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.value)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for List {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(ListVisitor::new())
    }
}

impl ListVisitor {
    pub fn new() -> ListVisitor {
        ListVisitor {}
    }
}

impl<'de> Visitor<'de> for ListVisitor {
    type Value = List;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON object property representing a scratch block")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        Ok(List {
            name: seq
                .next_element::<String>()
                .expect("enum variable array length is shorter than 2")
                .expect("cannot parse variable name"),
            value: seq
                .next_element::<Vec<PrimitiveValue>>()
                .expect("enum variable array length is shorter than 2")
                .expect("cannot parse variable value"),
        })
    }
}
