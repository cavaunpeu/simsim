use std::{error::Error, fs, collections::HashMap};
use serde_json::Value;

pub fn load_configs(path: String) -> Result<Vec<HashMap<String, f64>>, Box<dyn Error>> {
    let data = fs::read_to_string(&path).expect(&format!("{} not found", &path));
    let json: Value = serde_json::from_str(&data).expect("Could not read JSON");
    let configs = json.get("configs");
    match configs {
        None => panic!("No configs found"),
        Some(c) => Ok(
             c.as_array()
              .unwrap()
              .iter()
              .map(
                |c| c.as_object()
                     .unwrap()
                     .iter()
                     .map(|(k, v)| (k.to_owned(), v.as_f64().unwrap()))
                     .collect()
              )
              .collect::<Vec<_>>()
        )
    }
}
