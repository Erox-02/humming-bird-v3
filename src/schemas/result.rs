use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::schemas::{Entity, Pvdc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prcsres {
    pub original_text: String,
    pub masked_text: String,
    pub metadata: HashMap<String, String>,
    pub entities: Vec<Entity>,
    pub decisions: Vec<Pvdc>,
    pub has_pii: bool,
}

impl Prcsres {
    pub fn new(original: &str, masked: &str) -> Self {
        Self {
            original_text: original.to_string(),
            masked_text: masked.to_string(),
            metadata: HashMap::new(),
            entities: Vec::new(),
            decisions: Vec::new(),
            has_pii: false,
        }
    }

    pub fn to_dict(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("original_text".to_string(), serde_json::Value::String(self.original_text.clone()));
        map.insert("masked_text".to_string(), serde_json::Value::String(self.masked_text.clone()));
        map.insert("has_pii".to_string(), serde_json::Value::Bool(self.has_pii));
        
        let entities: Vec<HashMap<String, String>> = self.entities
            .iter()
            .map(|e| e.to_dict())
            .collect();
        map.insert("entities".to_string(), serde_json::to_value(entities).unwrap());
        
        map
    }
}