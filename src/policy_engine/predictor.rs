use crate::schemas::{Entity, Pvdc, DecisionType};

pub struct Pvprd {
    model: Option<Model>,
}

impl Pvprd {
    pub fn new() -> Self {
        Self { model: None }
    }

    pub fn predict_batch(
        &self,
        entities: &[Entity],
        text: &str,
        intent: Option<&str>,
    ) -> Vec<Pvdc> {
        entities
            .iter()
            .map(|entity| self.predict(entity, text, intent))
            .collect()
    }

    pub fn predict(
        &self,
        entity: &Entity,
        _text: &str,
        _intent: Option<&str>,
    ) -> Pvdc {
        let should_mask = self.should_mask_by_type(&entity.entity_type);

        Pvdc {
            entity: entity.clone(),
            decision: if should_mask { DecisionType::Mask } else { DecisionType::Keep },
            confidence: entity.confidence,
            context_string: None,
            reasoning: None,
        }
    }

    fn should_mask_by_type(&self, entity_type: &str) -> bool {
        match entity_type.to_uppercase().as_str() {
            "DATE" => false,
            _ => true,
        }
    }
}

impl Default for Pvprd {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Model;