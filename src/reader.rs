use serde::Deserialize;
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub struct DataRecord {
    #[serde(rename = "Master_Index")]
    #[serde(default)]
    pub master_index: Option<String>,
    #[serde(rename = "County")]
    #[serde(default)]
    pub county: Option<String>,
    #[serde(rename = "Health level")]
    #[serde(default)]
    pub health_level: Option<String>,
    #[serde(rename = "Years of Experience")]
    #[serde(default)]
    pub years_experience: Option<String>,
    #[serde(rename = "Prompt")]
    #[serde(default)]
    pub prompt: Option<String>,
    #[serde(rename = "Nursing Competency")]
    #[serde(default)]
    pub nursing_competency: Option<String>,
    #[serde(rename = "Clinical Panel")]
    #[serde(default)]
    pub clinical_panel: Option<String>,
    #[serde(rename = "Clinician")]
    #[serde(default)]
    pub clinician: Option<String>,
    #[serde(rename = "GPT4.0")]
    #[serde(default)]
    pub gpt4_0: Option<String>,
    #[serde(rename = "LLAMA")]
    #[serde(default)]
    pub llama: Option<String>,
    #[serde(rename = "GEMINI")]
    #[serde(default)]
    pub gemini: Option<String>,
    #[serde(rename = "DDX SNOMED")]
    #[serde(default)]
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
        
        // Basic validation
        if record.clinician.is_none() && record.gpt4_0.is_none() {
            eprintln!("Warning: Record with master_index {:?} has no target values", 
                record.master_index);
        }
        
        records.push(record);
    }

    if records.is_empty() {
        return Err("No records found in CSV file".into());
    }

    Ok(records)
}
