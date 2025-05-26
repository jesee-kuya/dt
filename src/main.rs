mod reader;
mod decision_tree;

use crate::reader::{read_records, DataRecord};
use decision_tree::{MultiTargetPredictor, TreeParams};
use serde::Serialize;
use std::{error::Error, fs::File, path::Path};

#[derive(Debug, Serialize)]
struct PredictionRecord {
    #[serde(rename = "Master_Index")]
    master_index: String,
    #[serde(rename = "Clinician")]
    clinician: Option<String>,
    #[serde(rename = "GPT4.0")]
    gpt4_0: Option<String>,
    #[serde(rename = "LLAMA")]
    llama: Option<String>,
    #[serde(rename = "GEMINI")]
    gemini: Option<String>,
    #[serde(rename = "DDX SNOMED")]
    ddx_snomed: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Load & dedupe training data
    let train_files = ["data/train.csv", "data/train_raw.csv"];
    let mut records = load_and_dedup(&train_files)?;
    println!("Unique training records: {}", records.len());

    // 2. Preprocess
    preprocess(&mut records);
    println!("Records after preprocessing: {}", records.len());

    // 3. Set pruning parameters
    let params = TreeParams {
        max_depth: 8,
        min_samples_leaf: 30,
        min_gain_ratio: 0.01,
    };

    // 4. Train with pruning
    let predictor = MultiTargetPredictor::build_with_params(&records, params);
    println!("Trained pruned multi-target predictor");

    // 5. Demo
    demo(&predictor);

    // 6. Predict on test set
    write_predictions(&predictor, Path::new("data/test.csv"), Path::new("predictions.csv"))?;
    println!("All done.");

    Ok(())
}

fn load_and_dedup<P: AsRef<Path>>(files: &[P]) -> Result<Vec<DataRecord>, Box<dyn Error>> {
    let mut all: Vec<DataRecord> = files
        .iter()
        .flat_map(|p| {
            let p = p.as_ref();
            println!("Loading {}", p.display());
            match read_records(p) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("  → failed {}: {}", p.display(), e);
                    Vec::new()
                }
            }
        })
        .collect();

    all.sort_by(|a, b| a.master_index.cmp(&b.master_index));
    all.dedup_by(|a, b| a.master_index == b.master_index);
    Ok(all)
}

fn preprocess(records: &mut [DataRecord]) {
    for rec in records.iter_mut() {
        if rec.years_experience.is_none() {
            rec.years_experience = Some("unknown".into());
        }
        if let Some(s) = rec.county.take() {
            rec.county = Some(s.to_lowercase());
        }
        if let Some(s) = rec.health_level.take() {
            rec.health_level = Some(s.to_lowercase());
        }
        if let Some(s) = rec.clinical_panel.take() {
            rec.clinical_panel = Some(s.to_lowercase());
        }
        if let Some(s) = rec.years_experience.take() {
            rec.years_experience = Some(s.to_lowercase());
        }
    }
}

fn demo(p: &MultiTargetPredictor) {
    let example = DataRecord {
        county: Some("Example County".into()),
        health_level: Some("High".into()),
        years_experience: Some("5".into()),
        clinical_panel: Some("Panel A".into()),
        ..Default::default()
    };
    let pred = p.predict(&example);
    println!("\nExample Predictions:");
    println!("  Clinician: {:?}", pred.clinician);
    println!("  GPT4.0:    {:?}", pred.gpt4_0);
    println!("  LLAMA:     {:?}", pred.llama);
    println!("  GEMINI:    {:?}", pred.gemini);
    println!("  SNOMED:    {:?}", pred.ddx_snomed);
}

fn write_predictions<P: AsRef<Path>>(
    predictor: &MultiTargetPredictor,
    input: P,
    output: P,
) -> Result<(), Box<dyn Error>> {
    println!("\nLoading test data from {}", input.as_ref().display());
    let tests = read_records(&input)?;
    println!("Loaded {} test records", tests.len());

    let file = File::create(output.as_ref())?;
    let mut wtr = csv::Writer::from_writer(file);

    for rec in tests {
        let pred = predictor.predict(&rec);
        let idx = rec.master_index.unwrap_or_else(|| "N/A".into());
        let row = PredictionRecord {
            master_index: idx,
            clinician: pred.clinician,
            gpt4_0: pred.gpt4_0,
            llama: pred.llama,
            gemini: pred.gemini,
            ddx_snomed: pred.ddx_snomed,
        };
        wtr.serialize(&row)?;
    }
    wtr.flush()?;
    println!("Predictions saved to {}", output.as_ref().display());
    Ok(())
}