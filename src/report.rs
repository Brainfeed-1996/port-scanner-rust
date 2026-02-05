use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanResult {
    pub port: u16,
    pub status: String,
    pub service_guess: String,
}

pub fn save_report(results: &Vec<ScanResult>, filename: &str) {
    let json = serde_json::to_string_pretty(results).unwrap();
    let mut file = File::create(filename).expect("Unable to create report file");
    file.write_all(json.as_bytes()).expect("Unable to write data");
    println!("[*] Report saved to {}", filename);
}