use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use serde_json::Value;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = &String::from("./phrases.txt");
    let phrases_file_path = args.get(2).unwrap_or(file_name);
    let phrases_to_remove = read_phrases_from_file(phrases_file_path)?;

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

fn remove_phrases_from_json(data: &mut Value, phrases: &[String]) {
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

fn read_phrases_from_file(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut phrases = Vec::new();
    for line in reader.lines() {
        let line = line?;
        phrases.push(line);
    }
    Ok(phrases)
}
