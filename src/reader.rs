use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;
use std::path::Path;

pub fn reader(path: &str) -> io::Result<()> {
    let path = Path::new(path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}