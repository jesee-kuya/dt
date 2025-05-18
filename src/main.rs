mod reader;
mod decision_tree;

use decision_tree::DecisionTree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/train.csv"; 
    println!("Attempting to read from: {}", path);
    
    let records = reader::reader(path)?;
    
    println!("\nSuccessfully loaded {} records", records.len());
    println!("First record clinical panel: {:?}", records[0].clinical_panel);
    
    // Build the decision tree
    let tree = DecisionTree::build(&records);
    println!("\nDecision tree built successfully!");
    
    
    // You can add code here to print or use the tree
    
    Ok(())
}