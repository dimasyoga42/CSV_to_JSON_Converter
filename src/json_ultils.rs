use std::error::Error;
use std::collections::HashMap;
use std::fs;
pub fn konversi(data_scv: &str) -> Result<String, Box<dyn Error >> {
    let mut rdr = csv::Reader::from_reader(data_scv.as_bytes());
    let mut records = Vec::new();
    for result in rdr.deserialize::<HashMap<String, String>>() {
        let record: HashMap<String, String> = result?;
        records.push(record);
    }

    let json_string = serde_json::to_string(&records)?;
    Ok(json_string)
}
pub fn simpan_json(path: &str, json_data: &str) -> Result<(), Box<dyn Error>> {
    fs::write(path, json_data)?;
    Ok(())
} 