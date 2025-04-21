// Copyright (c) 2025 rtpl Contributors
// SPDX-License-Identifier: MIT
// 
mod cli;
mod data;
mod template;

use anyhow::{Context, Result};
use cli::{load_defaults, parse_args, StdinType};
use data::{add_environment_to_context, load_data};
use template::{load_template, render_template, write_output};

fn main() -> Result<()> {
    let mut args = parse_args();
    load_defaults(&mut args).context("Failed to validate arguments")?;

    let template_content = load_template(
        args.template.as_deref(),
        args.stdin == Some(StdinType::Template),
    )
    .context("Failed to load template")?;

    let mut context = load_data(
        args.data_file.as_deref(),
        args.stdin == Some(StdinType::Data),
    )
    .context("Failed to load data")?;

    add_environment_to_context(&mut context);

    let rendered =
        render_template(&template_content, &context).context("Failed to render template")?;

    write_output(&rendered, args.output.as_deref()).context("Failed to write output")?;

    Ok(())
}
