mod reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "src/data/test.csv";
    let records = reader::reader(path)?;
    
    // Print first 2 records for verification
    println!("Loaded {} records", records.len());
    for (i, record) in records.iter().take(2).enumerate() {
        println!("Record {}: {:?}", i + 1, record);
    }
    
    Ok(())
}