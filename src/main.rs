mod reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/test.csv"; // Changed path
    println!("Attempting to read from: {}", path);
    
    let records = reader::reader(path)?;
    
    println!("\nSuccessfully loaded {} records", records.len());
    println!("First record clinical panel: {:?}", records[0].clinical_panel);
    
    Ok(())
}