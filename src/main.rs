use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use serde_json::Value;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let phrases_to_remove = vec![
        "As an AI assistant, ",
        "I'm an AI language model and "
    ];

    let dir_entries = fs::read_dir(&args[1])?;
    for entry in dir_entries {
        let file_path = entry?.path();
        let extension = file_path.extension().unwrap_or_default().to_string_lossy();

        if extension == "json" {
            let file = File::open(&file_path)?;
            let reader = BufReader::new(file);
            let mut data: Value = serde_json::from_reader(reader)?;

            remove_phrases_from_json(&mut data, &phrases_to_remove);

            let mut file = File::create(&file_path)?;
            file.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
        }
    }

    Ok(())
}

fn remove_phrases_from_json(data: &mut Value, phrases: &[&str]) {
    match data {
        Value::Object(map) => {
            for (_, value) in map.iter_mut() {
                remove_phrases_from_json(value, phrases);
            }
        },
        Value::Array(arr) => {
            for value in arr.iter_mut() {
                remove_phrases_from_json(value, phrases);
            }
        },
        Value::String(s) => {
            for phrase in phrases {
                *s = s.replace(phrase, "");
            }
        },
        _ => {}
    }
}
