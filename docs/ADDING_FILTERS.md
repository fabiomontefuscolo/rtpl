# Adding Custom Filters to RTPL

This guide explains how to add new custom filters to RTPL's Tera template engine.

## Overview

Filters in RTPL are functions that transform values in templates. They are applied using the pipe (`|`) syntax:

```jinja2
{{ variable | filter_name }}
{{ variable | filter_name(arg1, arg2) }}
```

All custom filters are defined in `src/filters.rs` to keep them organized and maintainable.

## Quick Start

To add a new filter, follow these steps:

1. **Write the filter function** in `src/filters.rs`
2. **Register the filter** in the `register_filters()` function
3. **Add tests** for the filter
4. **Update documentation**

## Step-by-Step Guide

### Step 1: Write the Filter Function

Create a new function in `src/filters.rs` with this signature:

```rust
fn my_filter_name(
    value: &TeraValue,
    args: &HashMap<String, TeraValue>,
) -> Result<TeraValue, TeraError> {
    // Your filter logic here
    Ok(TeraValue::String("result".to_string()))
}
```

**Parameters:**
- `value`: The input value being filtered (e.g., `{{ data | my_filter }}` passes `data`)
- `args`: Optional arguments passed to the filter (e.g., `{{ data | my_filter(key="value") }}`)

**Return:**
- `Result<TeraValue, TeraError>`: The transformed value or an error

### Step 2: Register the Filter

Add your filter to the `register_filters()` function:

```rust
pub fn register_filters(tera: &mut tera::Tera) {
    tera.register_filter("tojson", tojson);
    tera.register_filter("tojson_pretty", tojson_pretty);
    
    // Add your new filter here
    tera.register_filter("my_filter_name", my_filter_name);
}
```

### Step 3: Add Tests

Create comprehensive tests in the `#[cfg(test)]` module at the bottom of `src/filters.rs`:

```rust
#[test]
fn test_my_filter_name() {
    let mut tera = setup_tera();
    tera.add_raw_template("test", "{{ value | my_filter_name }}")
        .unwrap();

    let mut context = Context::new();
    context.insert("value", "input");

    let result = tera.render("test", &context).unwrap();

    assert_eq!(result, "expected_output");
}
```

**Test different scenarios:**
- ✅ Normal/happy path
- ✅ Edge cases (empty strings, null values, etc.)
- ✅ Error conditions
- ✅ With arguments (if applicable)

### Step 4: Document the Filter

Add comprehensive documentation comments to your filter function:

```rust
/// Brief description of what the filter does
///
/// More detailed explanation of the filter's behavior,
/// including any special cases or gotchas.
///
/// # Arguments
///
/// * `value` - Description of the input value
/// * `args` - Description of optional arguments:
///   - `key1`: What this argument does
///   - `key2`: What this argument does
///
/// # Returns
///
/// Description of what the filter returns
///
/// # Errors
///
/// Description of when/why the filter might fail
///
/// # Example
///
/// ```text
/// {{ variable | my_filter_name }}
/// # Output: transformed_value
///
/// {{ variable | my_filter_name(option="value") }}
/// # Output: differently_transformed_value
/// ```
fn my_filter_name(
    value: &TeraValue,
    args: &HashMap<String, TeraValue>,
) -> Result<TeraValue, TeraError> {
    // Implementation
}
```

## Complete Example: Adding a `uppercase` Filter

Here's a complete example of adding a simple `uppercase` filter:

### 1. Implementation

```rust
/// Converts a string to uppercase
///
/// This filter takes a string value and converts all characters to uppercase.
///
/// # Arguments
///
/// * `value` - The string to convert (must be a string)
/// * `_args` - No arguments accepted
///
/// # Returns
///
/// The uppercase version of the input string
///
/// # Errors
///
/// Returns an error if the value is not a string
///
/// # Example
///
/// ```text
/// {{ "hello" | uppercase }}
/// # Output: HELLO
/// ```
fn uppercase(
    value: &TeraValue,
    _args: &HashMap<String, TeraValue>,
) -> Result<TeraValue, TeraError> {
    match value {
        TeraValue::String(s) => Ok(TeraValue::String(s.to_uppercase())),
        _ => Err(TeraError::msg("uppercase filter requires a string")),
    }
}
```

### 2. Registration

```rust
pub fn register_filters(tera: &mut tera::Tera) {
    tera.register_filter("tojson", tojson);
    tera.register_filter("tojson_pretty", tojson_pretty);
    tera.register_filter("uppercase", uppercase);
}
```

### 3. Tests

```rust
#[test]
fn test_uppercase() {
    let mut tera = setup_tera();
    tera.add_raw_template("test", "{{ text | uppercase }}")
        .unwrap();

    let mut context = Context::new();
    context.insert("text", "hello world");

    let result = tera.render("test", &context).unwrap();

    assert_eq!(result, "HELLO WORLD");
}

#[test]
fn test_uppercase_with_non_string() {
    let mut tera = setup_tera();
    tera.add_raw_template("test", "{{ num | uppercase }}")
        .unwrap();

    let mut context = Context::new();
    context.insert("num", &123);

    let result = tera.render("test", &context);

    assert!(result.is_err());
}
```

## Working with Filter Arguments

Filters can accept arguments. Here's an example of a filter that pads a string:

```rust
/// Pads a string to a specified length
///
/// # Arguments
///
/// * `value` - The string to pad
/// * `args` - Optional arguments:
///   - `length`: Target length (default: 10)
///   - `char`: Padding character (default: " ")
///   - `side`: "left" or "right" (default: "right")
fn pad_string(
    value: &TeraValue,
    args: &HashMap<String, TeraValue>,
) -> Result<TeraValue, TeraError> {
    let string = match value {
        TeraValue::String(s) => s.clone(),
        _ => return Err(TeraError::msg("pad_string requires a string")),
    };

    // Extract arguments with defaults
    let length = args
        .get("length")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    let pad_char = args
        .get("char")
        .and_then(|v| v.as_str())
        .and_then(|s| s.chars().next())
        .unwrap_or(' ');

    let side = args
        .get("side")
        .and_then(|v| v.as_str())
        .unwrap_or("right");

    let result = match side {
        "left" => format!("{:>width$}", string, width = length),
        "right" => format!("{:<width$}", string, width = length),
        _ => return Err(TeraError::msg("side must be 'left' or 'right'")),
    };

    Ok(TeraValue::String(result))
}
```

Usage in templates:

```jinja2
{{ "hello" | pad_string(length=10, side="right") }}
# Output: "hello     "

{{ "hello" | pad_string(length=10, char="*", side="left") }}
# Output: "*****hello"
```

## Best Practices

### 1. Error Handling

Always provide clear error messages:

```rust
// ❌ Bad
Err(TeraError::msg("error"))

// ✅ Good
Err(TeraError::msg(format!(
    "tojson filter failed: expected object or array, got {}",
    value
)))
```

### 2. Type Checking

Handle different input types gracefully:

```rust
match value {
    TeraValue::String(s) => { /* handle string */ },
    TeraValue::Number(n) => { /* handle number */ },
    TeraValue::Bool(b) => { /* handle boolean */ },
    TeraValue::Null => { /* handle null */ },
    TeraValue::Array(a) => { /* handle array */ },
    TeraValue::Object(o) => { /* handle object */ },
}
```

### 3. Performance

For expensive operations, consider adding a note in the documentation:

```rust
/// Note: This filter performs a network request and may be slow
```

### 4. Naming Conventions

- Use lowercase with underscores: `to_json`, `pad_string`
- Make names descriptive and consistent with Jinja2 where applicable
- Avoid overly abbreviated names

### 5. Documentation

- Always include examples in doc comments
- Document all arguments and their defaults
- Explain edge cases and limitations
- Show expected output

## Testing Guidelines

### Essential Tests

Every filter should have tests for:

1. **Basic functionality** - Does it work with valid input?
2. **Edge cases** - Empty strings, null, zero, etc.
3. **Type errors** - What happens with wrong input types?
4. **Argument handling** - If it accepts arguments, test them

### Test Structure

```rust
#[test]
fn test_filter_basic() {
    // Test the happy path
}

#[test]
fn test_filter_edge_case() {
    // Test edge cases
}

#[test]
fn test_filter_error_handling() {
    // Test error conditions
}

#[test]
fn test_filter_with_args() {
    // Test with different arguments
}
```

## Common Patterns

### Converting Types

```rust
// String to number
let num = value.as_str()
    .and_then(|s| s.parse::<i64>().ok())
    .ok_or_else(|| TeraError::msg("Invalid number"))?;

// Number to string
let s = match value {
    TeraValue::Number(n) => n.to_string(),
    _ => return Err(TeraError::msg("Expected number")),
};
```

### Working with Collections

```rust
// Array operations
let array = value.as_array()
    .ok_or_else(|| TeraError::msg("Expected array"))?;

let result: Vec<TeraValue> = array
    .iter()
    .map(|item| /* transform item */)
    .collect();

Ok(TeraValue::Array(result))
```

### Using External Libraries

```rust
use some_crate::transform;

fn my_filter(value: &TeraValue, _args: &HashMap<String, TeraValue>) 
    -> Result<TeraValue, TeraError> 
{
    let input = value.as_str()
        .ok_or_else(|| TeraError::msg("Expected string"))?;
    
    let result = transform(input)
        .map_err(|e| TeraError::msg(format!("Transform failed: {}", e)))?;
    
    Ok(TeraValue::String(result))
}
```

## Updating Documentation

After adding a new filter, update the following files:

1. **`README.md`** - Add the filter to the "Custom Filters" section with usage example
2. **`src/filters.rs`** - Add comprehensive doc comments
3. **This guide** - Add complex filters as examples (optional)

## Running Tests

```bash
# Run all tests
cargo test

# Run only filter tests
cargo test filters::tests

# Run a specific test
cargo test test_my_filter_name

# Run tests with output
cargo test -- --nocapture
```

## Example: Real-World Filter

Here's a complete example of a useful filter that formats numbers with thousand separators:

```rust
/// Formats a number with thousand separators
///
/// Converts a number to a string with commas (or specified separator)
/// separating thousands.
///
/// # Arguments
///
/// * `value` - The number to format (string or number)
/// * `args` - Optional arguments:
///   - `separator`: The separator character (default: ",")
///   - `decimals`: Number of decimal places (default: preserve original)
///
/// # Example
///
/// ```text
/// {{ 1234567 | format_number }}
/// # Output: 1,234,567
///
/// {{ 1234.5 | format_number(decimals=2) }}
/// # Output: 1,234.50
///
/// {{ 1234567 | format_number(separator=".") }}
/// # Output: 1.234.567
/// ```
fn format_number(
    value: &TeraValue,
    args: &HashMap<String, TeraValue>,
) -> Result<TeraValue, TeraError> {
    // Parse the number
    let num = match value {
        TeraValue::Number(n) => n.as_f64()
            .ok_or_else(|| TeraError::msg("Invalid number"))?,
        TeraValue::String(s) => s.parse::<f64>()
            .map_err(|e| TeraError::msg(format!("Parse error: {}", e)))?,
        _ => return Err(TeraError::msg("Expected number or string")),
    };

    let separator = args
        .get("separator")
        .and_then(|v| v.as_str())
        .unwrap_or(",");

    let decimals = args
        .get("decimals")
        .and_then(|v| v.as_u64())
        .map(|d| d as usize);

    // Format the number
    let formatted = if let Some(dec) = decimals {
        format!("{:.prec$}", num, prec = dec)
    } else {
        num.to_string()
    };

    // Add thousand separators
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = parts.get(1);

    let mut result = String::new();
    for (i, c) in integer_part.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push_str(separator);
        }
        result.push(c);
    }
    result = result.chars().rev().collect();

    if let Some(dec) = decimal_part {
        result.push('.');
        result.push_str(dec);
    }

    Ok(TeraValue::String(result))
}
```

## Need Help?

- Check existing filters in `src/filters.rs` for examples
- Review Tera's documentation: https://keats.github.io/tera/
- Look at Jinja2 filters for inspiration: https://jinja.palletsprojects.com/en/stable/templates/#list-of-builtin-filters
- Open an issue or discussion on GitHub

## Summary Checklist

When adding a new filter:

- [ ] Write the filter function with proper signature
- [ ] Register it in `register_filters()`
- [ ] Add comprehensive doc comments
- [ ] Write at least 3-5 tests
- [ ] Test edge cases and error conditions
- [ ] Update README.md
- [ ] Run `cargo test`
- [ ] Run `cargo clippy`
- [ ] Run `cargo fmt`
- [ ] Commit with descriptive message

Happy filtering! 🎉