use anyhow::Result;

#[path = "../src/ml/mod.rs"]
mod ml;

use ml::dataset::load_dataset;
use ml::features::{extract_features, FEATURE_COUNT};
use ml::model::HbpModel;

fn main() -> Result<()> {
    let model = HbpModel::load("assets/hbp100-v3.lgb")?;
    let samples = load_dataset("dataset.json")?;

    println!("loaded {} samples", samples.len());
    println!("feature count = {}", FEATURE_COUNT);
    println!();

    let mut correct = 0usize;
    let total = samples.len().min(20);

    for sample in samples.iter().take(20) {
        let features = extract_features(sample);
        assert_eq!(
            features.len(),
            FEATURE_COUNT,
            "extract_features returned wrong length for entity_type={}",
            sample.entity_type
        );

        let probability = model.predict(&features)?;
        let prediction = if probability >= 0.5 { "MASK" } else { "KEEP" };

        let expected = format!("{:?}", sample.label);
        let hit = expected.eq_ignore_ascii_case(prediction);
        if hit {
            correct += 1;
        }

        println!(
            "[actual:{:>4}] [predicted:{:>4}] [prob:{:.4}] {:<8} {}",
            expected, prediction, probability, sample.entity_type, sample.entity_value
        );
    }

    println!();
    println!("first {}/{} correct on this sample", correct, total);
    Ok(())
}