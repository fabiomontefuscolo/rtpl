// Copyright (c) 2025 rtpl Contributors
// SPDX-License-Identifier: MIT
// 
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Read};

pub fn load_data(data_file: Option<&str>, from_stdin: bool) -> Result<Value> {
    match (data_file, from_stdin) {
        // JSON can com from file or stdin
        (Some(file), false) => load_json_from_file(file),
        (None, true) => load_json_from_stdin(),

        // Default to empty
        _ => Ok(Value::Object(serde_json::Map::new())),
    }
}

fn load_json_from_file(file_path: &str) -> Result<Value> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read JSON file: {}", file_path))?;

    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from file: {}", file_path))
}

fn load_json_from_stdin() -> Result<Value> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read JSON from stdin")?;

    serde_json::from_str(&buffer).context("Failed to parse JSON from stdin")
}

pub fn add_environment_to_context(context: &mut Value) {
    let env_vars: HashMap<String, Value> =
        env::vars().map(|(k, v)| (k, Value::String(v))).collect();

    if let Value::Object(ref mut map) = context {
        map.insert(
            "_ENV".to_string(),
            Value::Object(env_vars.into_iter().collect()),
        );
    }
}

/**
 * Unit tests
 */
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_data_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let json_content = r#"{"name": "test", "value": 42}"#;
        write!(temp_file, "{}", json_content).unwrap();

        let file_path = temp_file.path().to_str().unwrap();
        let result = load_data(Some(file_path), false).unwrap();

        let expected = json!({
            "name": "test",
            "value": 42
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_load_data_empty() {
        let result = load_data(None, false).unwrap();
        let expected = json!({});

        assert_eq!(result, expected);
    }
}
