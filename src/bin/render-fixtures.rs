// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Renders the retained evidence packet from fixture inputs (FR-003).

use std::path::Path;
use std::process::ExitCode;

use quire_verification::fixtures::{read_bounded_json, render_fixture_packet};

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
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let portfolio = read_bounded_json(&root.join("fixtures/portfolio.json"))?;
    let results = read_bounded_json(&root.join("fixtures/results.json"))?;
    Ok(serde_json::to_string_pretty(&render_fixture_packet(
        &portfolio, &results,
    )?)?)
}
