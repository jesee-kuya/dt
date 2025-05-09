mod reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "src/data/train.csv";
    let records = reader::reader(path)?;
    
    // Print first 2 records for verification
    for (i, record) in records.iter().take(2).enumerate() {
        println!("Record {}:", i + 1);
        println!("  County: {}", record.county);
        println!("  Clinical Panel: {}", record.clinical_panel);
        // Access other fields you need...
    }
    
    Ok(())
}