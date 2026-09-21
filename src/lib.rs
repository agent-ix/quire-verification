// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Qualified-path verification contracts, planning, and evidence assessment.
//!
//! Governing requirements: StR-001 and NFR-004.

#![forbid(unsafe_code)]

pub mod bounded_portfolio;
pub mod catalog;
/// Schema-owned public contracts, RFC 8785 canonicalization, and bounded JSON ingestion.
///
/// Extracted into public crate `quire-verification-contracts` (PLAT-861); re-exported here so
/// every internal call site, including the internal-only schema definitions reached through
/// `contracts::validate_schema_definition`, is unchanged.
pub use quire_verification_contracts as contracts;
pub mod evidence;
pub mod evidence_ledger;
pub mod fixtures;
pub mod hybrid_synthesis_provider;
pub mod linked_method_plan;
pub mod measurement;
pub mod model_testing;
pub mod native_process;
pub mod parser_portfolio;
pub mod planner;
pub mod qualification_corpus;
pub mod qualification_oracle;
pub mod qualification_oracle_execution;
pub mod qualification_profile;
pub mod qualification_technique;
pub mod relational_hyperproperty;
pub mod report;
pub mod shared_reference;

mod value;
