// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Validates every retained JSON fixture (FR-003).

use std::path::Path;
use std::process::ExitCode;

use quire_verification::fixtures::validate_all_fixtures;

fn main() -> ExitCode {
    match validate_all_fixtures(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .as_path(),
    ) {
        Ok(summary) => {
            let contracts: usize = summary.iter().map(|item| item.contracts).sum();
            println!(
                "validated {} fixtures and {contracts} public contract instances against e02-draft-2.schema.json",
                summary.len()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
