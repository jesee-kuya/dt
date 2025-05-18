mod reader;
mod decision_tree;

use crate::reader::DataRecord;
use decision_tree::MultiTargetPredictor;
use serde::Serialize;
use std::error::Error;

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
    // Load training data
    let train_path = "/home/jkuya/dt/data/train.csv";
    println!("Loading training data from: {}", train_path);
    let records = reader::reader(train_path)?;
    println!("Loaded {} training records", records.len());
    
    // Train the model
    let predictor = MultiTargetPredictor::build(&records);
    println!("Trained multi-target predictor");
    
    // Example prediction
    let test_record = DataRecord {
        county: Some("Example County".to_string()),
        health_level: Some("High".to_string()),
        years_experience: Some("5".to_string()),
        clinical_panel: Some("Panel A".to_string()),
        ..Default::default()
    };
    
    let prediction = predictor.predict(&test_record);
    println!("\nExample Predictions:");
    println!("Clinician: {:?}", prediction.clinician);
    println!("GPT4.0: {:?}", prediction.gpt4_0);
    println!("LLAMA: {:?}", prediction.llama);
    println!("GEMINI: {:?}", prediction.gemini);
    println!("DDX SNOMED: {:?}", prediction.ddx_snomed);
    
    // Load test data and generate predictions
    let test_path = "/home/jkuya/dt/data/test.csv";
    println!("\nLoading test data from: {}", test_path);
    let test_records = reader::reader(test_path)?;
    println!("Loaded {} test records", test_records.len());
    
    // Write predictions to CSV
    let mut wtr = csv::Writer::from_path("predictions.csv")?;
    for test_record in test_records {
        let prediction = predictor.predict(&test_record);
        let master_index = test_record.master_index.unwrap_or_else(|| "N/A".to_string());
        let pred_record = PredictionRecord {
            master_index,
            clinician: prediction.clinician,
            gpt4_0: prediction.gpt4_0,
            llama: prediction.llama,
            gemini: prediction.gemini,
            ddx_snomed: prediction.ddx_snomed,
        };
        wtr.serialize(&pred_record)?;
    }
    wtr.flush()?;
    println!("\nPredictions saved to predictions.csv");
    
    Ok(())
}