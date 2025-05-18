mod reader;
mod decision_tree;

use crate::reader::DataRecord;
use decision_tree::MultiTargetPredictor;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/home/jkuya/dt/data/train.csv";
    println!("Loading data from: {}", path);
    
    let records = reader::reader(path)?;
    println!("Loaded {} records", records.len());
    
    let predictor = MultiTargetPredictor::build(&records);
    println!("Trained multi-target predictor");
    
    // Example prediction
    let test_record = DataRecord {
        county: Some("Example County".to_string()),
        health_level: Some("High".to_string()),
        years_experience: Some("5".to_string()),
        clinical_panel: Some("Panel A".to_string()),
        // Populate other fields as needed
        ..Default::default()
    };
    
    let prediction = predictor.predict(&test_record);
    println!("\nPredictions:");
    println!("Clinician: {:?}", prediction.clinician);
    println!("GPT4.0: {:?}", prediction.gpt4_0);
    println!("LLAMA: {:?}", prediction.llama);
    println!("GEMINI: {:?}", prediction.gemini);
    println!("DDX SNOMED: {:?}", prediction.ddx_snomed);
    
    Ok(())
}