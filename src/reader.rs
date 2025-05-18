use serde::Deserialize;
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub struct DataRecord {
    #[serde(rename = "Master_Index")]
    pub master_index: Option<String>,
    #[serde(rename = "County")]
    pub county: Option<String>,
    #[serde(rename = "Health level")]
    pub health_level: Option<String>,
    #[serde(rename = "Years of Experience")]
    pub years_experience: Option<String>,
    #[serde(rename = "Prompt")]
    pub prompt: Option<String>,
    #[serde(rename = "Nursing Competency")]
    pub nursing_competency: Option<String>,
    #[serde(rename = "Clinical Panel")]
    pub clinical_panel: Option<String>,
    #[serde(rename = "Clinician")]
    pub clinician: Option<String>,
    #[serde(rename = "GPT4.0")]
    pub gpt4_0: Option<String>,
    #[serde(rename = "LLAMA")]
    pub llama: Option<String>,
    #[serde(rename = "GEMINI")]
    pub gemini: Option<String>,
    #[serde(rename = "DDX SNOMED")]
    pub ddx_snomed: Option<String>,
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
