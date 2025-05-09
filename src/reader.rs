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
        .from_path(path)?;

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: DataRecord = result?;
        records.push(record);
    }

    Ok(records)
}
