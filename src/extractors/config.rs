use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::schemas::Entity;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Excon {
    pub name: String,
    pub entity_type: String,
    pub pattern: String,
    pub confidence: Option<f32>,
}

pub struct Conex {
    config: Excon,
    regex: Regex,
}

impl Conex {
    pub fn new(config: Excon) -> Result<Self, String> {
        let regex = Regex::new(&config.pattern)
            .map_err(|e| format!("Invalid regex for '{}': {}", config.name, e))?;
        Ok(Self { config, regex })
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn entity_type(&self) -> &str {
        &self.config.entity_type
    }

    pub fn extract(&self, text: &str) -> Vec<Entity> {
        let confidence = self.config.confidence.unwrap_or(0.85);

        self.regex
            .find_iter(text)
            .map(|m| Entity {
                entity_type: self.config.entity_type.clone(),
                value: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
                confidence,
                placeholder: None,
                metadata: HashMap::new(),
            })
            .collect()
    }
}