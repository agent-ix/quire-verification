// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Compares a real Quoin method catalog from standard input with the pinned fixture (IT-001).

use std::io::Read;
use std::path::Path;
use std::process::ExitCode;

use quire_verification::contracts::parse_bounded_json;
use quire_verification::fixtures::{check_quoin_baseline, read_bounded_json};

fn main() -> ExitCode {
    match run() {
        Ok(count) => {
            println!("Quoin baseline parity: {count} methods");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = Vec::new();
    std::io::stdin()
        .take(u64::try_from(quire_verification::contracts::MAX_JSON_BYTES + 1).unwrap_or(u64::MAX))
        .read_to_end(&mut input)?;
    let actual = parse_bounded_json(&input)?;
    let fixture = read_bounded_json(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/quoin-catalog-export.json"),
    )?;
    Ok(check_quoin_baseline(&actual, &fixture)?)
}
