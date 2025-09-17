use crate::model::element::Comment;
use crate::model::target::CommentList;
use std::ops::{Deref, DerefMut};
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use serde::ser::SerializeMap;

struct CommentVisitor;

impl Serialize for CommentList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for b in self.0.clone().into_iter() {
            map.serialize_entry(&b.obj_id, &b)?;
        }
        map.end()
    }
}

impl Deref for CommentList {
    type Target = Vec<Comment>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CommentList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de> Deserialize<'de> for CommentList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        
        deserializer.deserialize_map(CommentVisitor::new())
    }
}

impl CommentVisitor {
    pub fn new() -> CommentVisitor {
        CommentVisitor {}
    }
}

impl<'de> Visitor<'de> for CommentVisitor {
    type Value = CommentList;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON object property representing a scratch Comment")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        
        let mut output: CommentList = CommentList(Vec::new());
        while let Some((id, content)) = map.next_entry::<String, serde_json::Value>()? {
            let mut b: Comment = serde_json::from_value(content).unwrap();
            b.obj_id = id;
            output.push(b);
        }
        
        Ok(output)
    }
}