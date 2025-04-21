// Copyright (c) 2025 rtpl Contributors
// SPDX-License-Identifier: MIT
// 
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::io::{self, Read, Write};
use tera::{Context as TeraContext, Tera};

pub fn load_template(template_file: Option<&str>, from_stdin: bool) -> Result<String> {
    match (template_file, from_stdin) {
        // file is there and stdin is NOT
        (Some(file), false) => read_template_from_file(file),

        // file is there and stdin is there
        (Some(_), true) => Err(anyhow::anyhow!(
            "Cannot read template from both file and stdin"
        )),

        // file is NOT there and stdin is there
        (None, true) => read_template_from_stdin(),

        // file is NOT there and stdin is NOT there
        (None, false) => Err(anyhow::anyhow!("No template file provided")),
    }
}

pub fn read_template_from_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read template from stdin")?;
    Ok(buffer)
}

pub fn read_template_from_file(template_file: &str) -> Result<String> {
    let mut buffer = String::new();
    fs::File::open(template_file)
        .with_context(|| format!("Failed to open template file: {}", template_file))?
        .read_to_string(&mut buffer)
        .context("Failed to read template from file")?;
    Ok(buffer)
}

pub fn render_template(template_content: &str, context: &Value) -> Result<String> {
    let mut tera = Tera::default();

    // Register the template with a name
    tera.add_raw_template("template", template_content)
        .context("Failed to add template to Tera")?;

    // Convert serde_json::Value to TeraContext
    let context = match context {
        Value::Object(map) => {
            let mut tera_context = TeraContext::new();
            for (key, value) in map {
                tera_context.insert(key, value);
            }
            tera_context
        }
        _ => TeraContext::new(),
    };

    // Render the template
    tera.render("template", &context)
        .context("Failed to render template")
}

pub fn write_output(content: &str, output_file: Option<&str>) -> Result<()> {
    if let Some(file) = output_file {
        fs::write(file, content)
            .with_context(|| format!("Failed to write to output file: {}", file))?;
    } else {
        io::stdout()
            .write_all(content.as_bytes())
            .context("Failed to write to stdout")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_template_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let template_content = "Hello {{ name }}!";
        write!(temp_file, "{}", template_content).unwrap();
        
        let file_path = temp_file.path().to_str().unwrap();
        
        let result = load_template(Some(file_path), false).unwrap();
        
        assert_eq!(result, template_content);
    }

    #[test]
    fn test_render_template() {
        let template_content = "Hello {{ name }}!";
        
        let context = json!({
            "name": "World"
        });
        
        let result = render_template(template_content, &context).unwrap();
        
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_render_template_with_env() {
        let template_content = "User: {{ _ENV.USER }}";
        
        let context = json!({
            "_ENV": {
                "USER": "test_user"
            }
        });
        
        let result = render_template(template_content, &context).unwrap();
        
        assert_eq!(result, "User: test_user");
    }

    #[test]
    fn test_write_output_to_file() {
        let content = "Test output";
        
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();
        
        write_output(content, Some(file_path)).unwrap();
        
        let result = fs::read_to_string(file_path).unwrap();
        
        assert_eq!(result, content);
    }
}
