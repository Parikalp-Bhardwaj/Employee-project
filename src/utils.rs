use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};

use crate::model::List;

pub fn load_from_json(file_path: &str) -> Result<List, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let list: List = serde_json::from_str(&json_data)
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

    Ok(list)
}

