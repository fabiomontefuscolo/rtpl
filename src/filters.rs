// Copyright (c) 2025 rtpl Contributors
// SPDX-License-Identifier: MIT
//
//! Custom filters for Tera template engine
//!
//! This module provides custom filters that extend Tera's built-in functionality
//! to match Jinja2 behavior and add useful transformations.
//!
//! ## Adding New Filters
//!
//! To add a new custom filter:
//!
//! 1. **Create the filter function** with this signature:
//!    ```ignore
//!    fn my_filter(
//!        value: &TeraValue,
//!        args: &HashMap<String, TeraValue>
//!    ) -> Result<TeraValue, TeraError>
//!    ```
//!
//! 2. **Register it** in the `register_filters` function:
//!    ```ignore
//!    tera.register_filter("my_filter", my_filter);
//!    ```
//!
//! 3. **Add tests** to verify the filter works correctly
//!
//! 4. **Document** the filter with doc comments explaining:
//!    - What the filter does
//!    - Arguments it accepts
//!    - Return value
//!    - Usage examples
//!
//! ## Available Filters
//!
//! - **`tojson`** - Converts a value to JSON format (supports optional `indent` parameter)
//!
//! ## Example
//!
//! ```rust
//! use tera::Tera;
//! use rtpl::filters::register_filters;
//!
//! let mut tera = Tera::default();
//! register_filters(&mut tera);
//!
//! // Now you can use custom filters in templates
//! tera.add_raw_template("example", "{{ data | tojson }}").unwrap();
//! ```

use serde::Serialize;
use std::collections::HashMap;
use tera::{Error as TeraError, Value as TeraValue};

/// Register all custom filters with a Tera instance
///
/// # Arguments
///
/// * `tera` - Mutable reference to a Tera instance
///
/// # Example
///
/// ```
/// use tera::Tera;
/// use rtpl::filters::register_filters;
///
/// let mut tera = Tera::default();
/// register_filters(&mut tera);
/// ```
pub fn register_filters(tera: &mut tera::Tera) {
    // JSON filters (Jinja2-compatible)
    tera.register_filter("tojson", tojson);

    // Add more custom filters here following the same pattern:
    // tera.register_filter("filter_name", filter_function);
}

/// Converts a value to JSON format
///
/// This filter mimics Jinja2's `tojson` filter, converting any value to its
/// JSON representation. By default, produces compact JSON without extra whitespace.
/// With the `indent` parameter, produces pretty-printed JSON with indentation.
///
/// # Arguments
///
/// * `value` - The value to convert to JSON
/// * `args` - Filter arguments:
///   - `indent`: Optional number of spaces for indentation (enables pretty-printing)
///
/// # Returns
///
/// A Tera Value containing the JSON string representation
///
/// # Errors
///
/// Returns a TeraError if the value cannot be serialized to JSON
///
/// # Examples
///
/// ```text
/// {{ data | tojson }}
/// # Output: {"key":"value","number":123}
///
/// {{ data | tojson(indent=2) }}
/// # Output:
/// # {
/// #   "key": "value",
/// #   "number": 123
/// # }
/// ```
fn tojson(value: &TeraValue, args: &HashMap<String, TeraValue>) -> Result<TeraValue, TeraError> {
    // Check if indent parameter is provided
    let indent = args.get("indent").and_then(|v| v.as_u64());

    let json_string = if let Some(indent_size) = indent {
        // Pretty-print with specified indentation
        let indent_bytes = vec![b' '; indent_size as usize];
        let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
        let mut buf = Vec::new();
        let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formatter);
        value
            .serialize(&mut serializer)
            .map_err(|e| TeraError::msg(format!("Failed to convert to JSON: {}", e)))?;
        String::from_utf8(buf)
            .map_err(|e| TeraError::msg(format!("Invalid UTF-8 in JSON: {}", e)))?
    } else {
        // Compact format
        serde_json::to_string(value)
            .map_err(|e| TeraError::msg(format!("Failed to convert to JSON: {}", e)))?
    };

    Ok(TeraValue::String(json_string))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tera::{Context, Tera};

    fn setup_tera() -> Tera {
        let mut tera = Tera::default();
        register_filters(&mut tera);
        tera
    }

    #[test]
    fn test_tojson_with_object() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ data | tojson }}")
            .unwrap();

        let mut context = Context::new();
        context.insert(
            "data",
            &json!({
                "name": "test",
                "value": 123,
                "array": [1, 2, 3]
            }),
        );

        let result = tera.render("test", &context).unwrap();

        // Parse the result to verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["name"], "test");
        assert_eq!(parsed["value"], 123);
        assert_eq!(parsed["array"], json!([1, 2, 3]));
    }

    #[test]
    fn test_tojson_with_string() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ name | tojson }}")
            .unwrap();

        let mut context = Context::new();
        context.insert("name", "hello world");

        let result = tera.render("test", &context).unwrap();

        assert_eq!(result, "\"hello world\"");
    }

    #[test]
    fn test_tojson_with_number() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ num | tojson }}").unwrap();

        let mut context = Context::new();
        context.insert("num", &42);

        let result = tera.render("test", &context).unwrap();

        assert_eq!(result, "42");
    }

    #[test]
    fn test_tojson_with_array() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ items | tojson }}")
            .unwrap();

        let mut context = Context::new();
        context.insert("items", &vec!["a", "b", "c"]);

        let result = tera.render("test", &context).unwrap();

        assert_eq!(result, "[\"a\",\"b\",\"c\"]");
    }

    #[test]
    fn test_tojson_with_indent() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ data | tojson(indent=2) }}")
            .unwrap();

        let mut context = Context::new();
        context.insert(
            "data",
            &json!({
                "name": "test",
                "value": 123
            }),
        );

        let result = tera.render("test", &context).unwrap();

        // Verify it contains newlines (pretty-printed)
        assert!(result.contains('\n'));

        // Parse the result to verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["name"], "test");
        assert_eq!(parsed["value"], 123);
    }

    #[test]
    fn test_tojson_with_indent_has_proper_indentation() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ data | tojson(indent=2) }}")
            .unwrap();

        let mut context = Context::new();
        context.insert("data", &json!({"nested": {"key": "value"}}));

        let result = tera.render("test", &context).unwrap();

        // Verify it has proper indentation (2 spaces)
        assert!(result.contains("  \"nested\":"));
        assert!(result.contains("    \"key\":"));
    }

    #[test]
    fn test_tojson_with_different_indent_size() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ data | tojson(indent=4) }}")
            .unwrap();

        let mut context = Context::new();
        context.insert("data", &json!({"key": "value"}));

        let result = tera.render("test", &context).unwrap();

        // Verify it has 4-space indentation
        assert!(result.contains("    \"key\":"));
    }

    #[test]
    fn test_tojson_without_indent_is_compact() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ data | tojson }}")
            .unwrap();

        let mut context = Context::new();
        context.insert("data", &json!({"key": "value", "num": 123}));

        let result = tera.render("test", &context).unwrap();

        // Verify it does NOT contain newlines (compact)
        assert!(!result.contains('\n'));
        assert!(
            result.contains("{\"key\":\"value\",\"num\":123}")
                || result.contains("{\"num\":123,\"key\":\"value\"}")
        );
    }

    #[test]
    fn test_tojson_escapes_special_characters() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ text | tojson }}")
            .unwrap();

        let mut context = Context::new();
        context.insert("text", "Hello \"World\"\nNew line");

        let result = tera.render("test", &context).unwrap();

        // Verify proper JSON escaping
        assert!(result.contains("\\\""));
        assert!(result.contains("\\n"));
    }

    #[test]
    fn test_tojson_with_boolean() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ flag | tojson }}")
            .unwrap();

        let mut context = Context::new();
        context.insert("flag", &true);

        let result = tera.render("test", &context).unwrap();

        assert_eq!(result, "true");
    }

    #[test]
    fn test_tojson_with_null() {
        let mut tera = setup_tera();
        tera.add_raw_template("test", "{{ value | tojson }}")
            .unwrap();

        let mut context = Context::new();
        context.insert("value", &serde_json::Value::Null);

        let result = tera.render("test", &context).unwrap();

        assert_eq!(result, "null");
    }
}
