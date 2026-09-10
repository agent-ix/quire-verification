#![no_main]

//! Fuzz target for bounded native-process response decoding.

use ix_trace_rs::trace;
use libfuzzer_sys::fuzz_target;
use quire_verification::native_process::{NativeProcessOutput, parse_native_json_output};

/// The native response reader refuses arbitrary valid Unicode output without panicking.
///
/// Trace: NFR-006-AC-5
// spec-fuzz: obligation=NFR-006-AC-5 harness=cargo-fuzz entry=native_process::parse_native_json_output origin=advised
#[trace("TC-038", "NFR-006-AC-5", "TC-022", "NFR-004-AC-3")]
fn fuzz_nfr_006_ac_5_native_response(data: &[u8]) {
    if let Ok(stdout) = std::str::from_utf8(data) {
        let output = NativeProcessOutput {
            exit_code: 0,
            stdout: stdout.to_owned(),
            stderr: String::new(),
        };
        let _ = parse_native_json_output(&output);
    }
}

fuzz_target!(|data: &[u8]| {
    fuzz_nfr_006_ac_5_native_response(data);
});
