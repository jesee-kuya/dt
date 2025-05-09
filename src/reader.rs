use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct DataRecord {
    #[serde(rename = "Master_Index")]
    master_index: String,
    #[serde(rename = "County")]
    county: String,
    #[serde(rename = "Health level")]
    health_level: String,
    #[serde(rename = "Years of Experience")]
    years_experience: f64,
    #[serde(rename = "Prompt")]
    prompt: String,
    #[serde(rename = "Nursing Competency")]
    nursing_competency: String,
    #[serde(rename = "Clinical Panel")]
    clinical_panel: String,
    #[serde(rename = "Clinician")]
    clinician: String,
    #[serde(rename = "GPT4.0")]
    gpt4_0: String,
    #[serde(rename = "LLAMA")]
    llama: String,
    #[serde(rename = "GEMINI")]
    gemini: String,
    #[serde(rename = "DDX SNOMED")]
    ddx_snomed: String,
}

pub fn reader(path: &str) -> Result<Vec<DataRecord>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: DataRecord = result?;
        records.push(record);
    }

    Ok(records)
}
