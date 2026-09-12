use crate::extractors::config::{Conex, Excon};
use crate::schemas::Entity;
use std::collections::HashSet;
use log;

pub struct Exman {
    extractors: Vec<Conex>,
    enabled: HashSet<String>,
}

impl Exman {
    pub fn new() -> Self {
        Self {
            extractors: Vec::new(),
            enabled: HashSet::new(),
        }
    }

    pub fn add_conex(&mut self, config: Excon) -> Result<(), String> {
        let extractor = Conex::new(config)?;
        self.enabled.insert(extractor.clone());
        self.extractors.push(extractor);
        log::info!("registered extractor {}", extractor.name());
        Ok(())
    }

    pub fn add_conjson(&mut self, json: &str) -> Result<(), String> {
        let config: Excon = serde_json::from_str(json)
            .map_err(|e| format!("invalid json {}", e))?;
        self.add_conex(config)
    }

    pub fn add_conf(&mut self, path: &str) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("failed to read {}", e))?;
        self.add_conjson(&content)
    }

    pub fn enex(&mut self, name: &str) -> bool {
        if self.extractors.iter().any(|e| e.name() == name) {
            self.enabled.insert(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn dis_ex(&mut self, name: &str) -> bool {
        self.enabled.remove(name)
    }

    pub fn ena(&self, name: &str) -> bool {
        self.enabled.contains(name)
    }

    pub fn ls_ex(&self) -> Vec<String> {
        self.extractors.iter().map(|e| e.name().to_string()).collect()
    }

    pub fn lsen(&self) -> Vec<String> {
        self.extractors
            .iter()
            .filter(|e| self.enabled.contains(e.name()))
            .map(|e| e.name().to_string())
            .collect()
    }

    pub fn exall(&self, text: &str) -> Vec<Entity> {
        let mut all_entities: Vec<Entity> = Vec::new();
        let mut seen_values = HashSet::new();

        for extractor in &self.extractors {
            if !self.enabled.contains(extractor.name()) {
                continue;
            }
            for entity in extractor.extract(text) {
                if seen_values.insert(entity.value.clone()) {
                    all_entities.push(entity);
                }
            }
        }

        all_entities.sort_by(|a, b| {
            a.start.cmp(&b.start)
                .then_with(|| (b.end - b.start).cmp(&(a.end - a.start)))
        });

        let mut filtered: Vec<Entity> = Vec::new();
        for entity in all_entities {
            let overlaps = filtered.iter().any(|kept| {
                !(entity.end <= kept.start || entity.start >= kept.end)
            });
            if !overlaps {
                filtered.push(entity);
            }
        }

        filtered
    }

    pub fn rst_def(&mut self) {
        self.extractors.clear();
        self.enabled.clear();
    }

    pub fn excnt(&self) -> usize {
        self.extractors.len()
    }
}

impl Default for Exman {
    fn default() -> Self {
        Self::new()
    }
}