use serde::Deserialize;
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DataRecord {
    #[serde(rename = "Master_Index")]
    pub master_index: String,
    #[serde(rename = "County")]
    pub county: String,
    #[serde(rename = "Health level")]
    pub health_level: String,
    #[serde(rename = "Years of Experience")]
    pub years_experience: f64,
    #[serde(rename = "Prompt")]
    pub prompt: String,
    #[serde(rename = "Nursing Competency")]
    pub nursing_competency: String,
    #[serde(rename = "Clinical Panel")]
    pub clinical_panel: String,
    #[serde(rename = "Clinician")]
    pub clinician: String,
    #[serde(rename = "GPT4.0")]
    pub gpt4_0: String,
    #[serde(rename = "LLAMA")]
    pub llama: String,
    #[serde(rename = "GEMINI")]
    pub gemini: String,
    #[serde(rename = "DDX SNOMED")]
    pub ddx_snomed: String,
}

pub fn reader(path: &str) -> Result<Vec<DataRecord>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .flexible(true)
        .from_path(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: DataRecord = result.map_err(|e| format!("CSV parsing error: {}", e))?;
        records.push(record);
    }

    if records.is_empty() {
        return Err("No records found in CSV file".into());
    }

    Ok(records)
}
