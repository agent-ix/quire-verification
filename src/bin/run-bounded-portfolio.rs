// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Runs the bounded local catalog portfolio (FR-009).

use std::path::Path;
use std::process::ExitCode;

use quire_verification::bounded_portfolio::run_bounded_portfolio;
use quire_verification::fixtures::read_bounded_json;

fn main() -> ExitCode {
    match run() {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<String, Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 {
        return Err("usage: run-bounded-portfolio <source-revision> <produced-at>".into());
    }
    let catalog = read_bounded_json(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/quoin-catalog-export.json"),
    )?;
    Ok(serde_json::to_string_pretty(&run_bounded_portfolio(
        &arguments[0],
        &arguments[1],
        &catalog,
    )?)?)
}
