// Copyright (c) 2025 rtpl Contributors
// SPDX-License-Identifier: MIT
// 
use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum StdinType {
    Template,
    Data,
    None,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub template: Option<String>,

    #[arg(long, value_enum)]
    pub stdin: Option<StdinType>,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long)]
    pub data_file: Option<String>,
}

pub fn load_defaults(args: &mut Args) -> Result<()> {
    if args.stdin.is_none() {
        if args.template.is_some() && args.data_file.is_some() {
            args.stdin = Some(StdinType::None);
        } else if args.template.is_some() {
            args.stdin = Some(StdinType::Data);
        } else {
            args.stdin = Some(StdinType::Template);
        }
    }

    if args.template.is_none() && args.stdin != Some(StdinType::Template) {
        return Err(anyhow!("No template provided"));
    }

    Ok(())
}

pub fn parse_args() -> Args {
    Args::parse()
}

/*
 * Unit tests
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_with_template_only() {
        let mut args = Args {
            template: Some("test.j2".to_string()),
            data_file: None,
            output: None,
            stdin: None,
        };

        let expected_stdin = Some(StdinType::Data);
        let result = load_defaults(&mut args);

        assert!(result.is_ok());
        assert_eq!(args.stdin, expected_stdin);
    }

    #[test]
    fn test_args_with_no_template() {
        let mut args = Args {
            template: None,
            data_file: None,
            output: None,
            stdin: Some(StdinType::Template),
        };

        let result = load_defaults(&mut args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_args_with_no_template_error() {
        let mut args = Args {
            template: None,
            data_file: None,
            output: None,
            stdin: Some(StdinType::Data),
        };

        let result = load_defaults(&mut args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No template provided");
    }

    #[test]
    fn test_args_with_template_and_data() {
        let mut args = Args {
            template: Some("test.j2".to_string()),
            data_file: Some("data.json".to_string()),
            output: None,
            stdin: None,
        };

        let expected_stdin = Some(StdinType::None);
        let result = load_defaults(&mut args);

        assert!(result.is_ok());
        assert_eq!(args.stdin, expected_stdin);
    }
}
