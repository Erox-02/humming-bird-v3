```rust
use hbp100::interfaces::EntityExtractor;
use hbp100::schemas::{Entity, EntityType};

pub struct MyExtractor;

impl EntityExtractor for MyExtractor {
    fn name(&self) -> &str {
        "MyExtractor"
    }

    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::Custom]
    }

    fn extract(&self, text: &str) -> Vec<Entity> {
        // write the regex here
        vec![]
    }
}
```
This is the basic syntax to create an extractor for this .
after creating a extractor , add it to mod.rs .