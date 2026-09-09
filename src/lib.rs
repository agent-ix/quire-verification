// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Qualified-path verification contracts, planning, and evidence assessment.
//!
//! Governing requirements: StR-001 and NFR-004.

#![forbid(unsafe_code)]

pub mod bounded_portfolio;
pub mod catalog;
pub mod contracts;
pub mod evidence;
pub mod fixtures;
pub mod native_process;
pub mod planner;
pub mod shared_reference;

mod value;
