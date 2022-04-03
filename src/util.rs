use csv;
use std::{error::Error, str::FromStr};
use serde::Deserialize;
use std::vec::Vec;

use solana_sdk::{
    pubkey::Pubkey,
    // signature::{read_keypair_file, Keypair}
};

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    pub address: String,
    pub amount: String,
}

pub fn get_pub(pubkey: &str) -> Pubkey {
    Pubkey::from_str(pubkey).unwrap()
}

pub fn read_from_file(path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve and print header record
    let headers = reader.headers()?;
    println!("{:?}", headers);

    let mut res = Vec::new();
    for result in reader.deserialize() {
        let record: Record = result?;
        
        let rec: Record = Record {
            address: record.address,
            amount: record.amount
        };
        res.push(rec);
    }

    Ok(res)
}