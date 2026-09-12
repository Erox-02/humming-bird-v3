use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    pub entity_type: String,
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub confidence: f32,
    pub placeholder: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl Entity {
    pub fn new(
        entity_type: String,
        value: String,
        start: usize,
        end: usize,
        confidence: f32,
    ) -> Self {
        Self {
            entity_type,
            value,
            start,
            end,
            confidence,
            placeholder: None,
            metadata: HashMap::new(),
        }
    }

    pub fn to_dict(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("type".to_string(), self.entity_type.clone());
        map.insert("value".to_string(), self.value.clone());
        map.insert("start".to_string(), self.start.to_string());
        map.insert("end".to_string(), self.end.to_string());
        map.insert("confidence".to_string(), self.confidence.to_string());
        map
    }
}