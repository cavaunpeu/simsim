use std::{error::Error, fs};
use serde_json::{Value, Map};

pub fn load_configs(path: String) -> Result<Vec<Map<String, Value>>, Box<dyn Error>> {
    let data = fs::read_to_string(&path).expect(&format!("{} not found", &path));
    let json: Value = serde_json::from_str(&data).expect("Could not read JSON");
    let configs = json.get("configs");
    match configs {
        None => panic!("No configs found"),
        Some(cfgs) => Ok(
            cfgs.as_array()
                .unwrap()
                .iter()
                .map(|c| c.as_object().unwrap().to_owned())
                .collect::<Vec<_>>()
        )
    }
}
