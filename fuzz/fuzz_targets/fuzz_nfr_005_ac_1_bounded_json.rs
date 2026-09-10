#![no_main]

use ix_trace_rs::trace;
use libfuzzer_sys::fuzz_target;
use quire_verification::contracts::parse_bounded_json;

/// The bounded public JSON reader refuses arbitrary bytes without panicking.
///
/// Trace: NFR-005-AC-1
///
/// spec-fuzz: obligation=NFR-005-AC-1 harness=cargo-fuzz entry=contracts::parse_bounded_json origin=advised
#[trace("TC-023", "NFR-005-AC-1", "TC-022", "NFR-004-AC-3")]
fn fuzz_nfr_005_ac_1_bounded_json(data: &[u8]) {
    let _ = parse_bounded_json(data);
}

fuzz_target!(|data: &[u8]| {
    fuzz_nfr_005_ac_1_bounded_json(data);
});
