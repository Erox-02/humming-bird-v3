use crate::schemas::{Entity, Pvdc};
use std::collections::HashMap;

pub trait Pvprd: Send + Sync {
    fn predict(
        &self,
        entity: &Entity,
        original_text: &str,
        intent: Option<&str>,
    ) -> Pvdc;
    
    fn predict_batch(
        &self,
        entities: &[Entity],
        original_text: &str,
        intent: Option<&str>,
    ) -> Vec<Pvdc>;
    
    fn load_assets(&mut self) -> bool;
    
    fn is_loaded(&self) -> bool;
    
    fn gt_md(&self) -> HashMap<String, String>;
}
