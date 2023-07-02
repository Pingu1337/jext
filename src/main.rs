use colored_json::ToColoredJson;
use serde_json::Result;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the user provided the JSON file path and a key
    if args.len() < 3 {
        println!("Usage: json_extractor <json_file> <key>");
        return Ok(());
    }

    // Extract the JSON file path and the key
    let json_file: &String = &args[1];
    let key: &String = &args[2];

    // Read the JSON file
    let mut file: File = match File::open(json_file) {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening file: {}", error);
            return Ok(());
        }
    };

    let mut contents: String = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", error);
        return Ok(());
    }

    // Parse the JSON
    let json: serde_json::Value = match serde_json::from_str::<serde_json::Value>(&contents) {
        Ok(json) => json,
        Err(error) => {
            println!("Error parsing JSON: {}", error);
            return Ok(());
        }
    };

    // Extract the value associated with the key
    let mut value: Option<&serde_json::Value> = Some(&json);
    for nested_key in key.split('.') {
        value = value.and_then(|v| v.get(nested_key));
    }

    // Print the value or an error message with syntax highlighting
    match value {
        Some(value) => {
            let colored_json: String = value.to_string().to_colored_json_auto()?;
            println!("{}", colored_json);
        }
        None => println!("Key not found: {}", key),
    }

    Ok(())
}
