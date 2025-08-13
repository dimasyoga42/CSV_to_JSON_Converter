use std::error::Error;
use std::env;
mod csv_ultils;
mod json_ultils;

fn main() {
    if let Err(err) = run() {
        eprintln!("❌ Error: {}", err);
        std::process::exit(1);
    }
}

pub fn run () -> Result<(), Box<dyn Error>> {
    let arg: Vec<String> = env::args().collect();
    if arg.len() != 3 {
        return Err("usage cargo run data.csv output.json".into());
    }; 
    let input_path = &arg[1];
    let input_output = &arg[2];
    let csv_data = csv_ultils::baca_csv(&input_path)?;
    let json_data = json_ultils::konversi(&csv_data)?;
    json_ultils::simpan_json(&input_output, &json_data)?;

    println!("konversi berhasil: {}", input_output);
    Ok(())
}