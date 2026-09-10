#![no_main]

//! Fuzz target for strict shared-reference JSON ingestion.

use ix_trace_rs::trace;
use libfuzzer_sys::fuzz_target;
use quire_verification::shared_reference::parse_strict_bounded_json;

/// The strict shared-reference reader refuses arbitrary bytes without panicking.
///
/// Trace: FR-016-AC-3
// spec-fuzz: obligation=FR-016-AC-3 harness=cargo-fuzz entry=shared_reference::parse_strict_bounded_json origin=advised
#[trace("TC-029", "FR-016-AC-3", "TC-022", "NFR-004-AC-3")]
fn fuzz_fr_016_ac_3_strict_shared_reference(data: &[u8]) {
    let _ = parse_strict_bounded_json(data);
}

fuzz_target!(|data: &[u8]| {
    fuzz_fr_016_ac_3_strict_shared_reference(data);
});
