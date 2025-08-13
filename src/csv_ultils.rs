use std::error::Error;
use std::fs;

pub fn baca_csv(path: &str) -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    Ok(data)
}
