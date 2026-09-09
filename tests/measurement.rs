// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! VP04 measurement acceptance tests.

use ix_trace_rs::trace;
use quire_verification::contracts::{VerificationErrorCode, wire};
use quire_verification::measurement::{
    MAX_ELAPSED_NANOSECONDS, measurement_observation, record_measurement_series,
    validate_measurement_series,
};
use serde_json::{Value, json};

const CANDIDATE: &str = "sha256:vp04-candidate";
const ENVIRONMENT: &str = "vp04-linux-environment";

fn context() -> Value {
    json!({
        "population": "16 observations for this exact candidate and fixture pair",
        "exclusions": ["build time", "other fixtures", "human review latency"],
        "clock": "monotonic-process-local",
        "commandOrder": ["setup", "direct-parse", "parse-format-reparse", "complete-run"],
        "cacheTreatment": "naturally warmed; neither cleared nor pre-warmed",
        "environmentalDifferences": [],
        "uncertainty": "scheduler, filesystem cache, clock granularity, and thermal state",
        "alternativeExplanations": ["filesystem cache state", "process scheduling"],
    })
}

fn observations() -> Vec<Value> {
    let mut values =
        vec![measurement_observation("setup-0", "setup", 0, "100", None).expect("setup")];
    for repetition in 1_u64..=5 {
        values.push(
            measurement_observation(
                &format!("direct-{repetition}"),
                "direct-parse",
                repetition,
                &format!("{}", repetition * 10),
                None,
            )
            .expect("direct"),
        );
        values.push(
            measurement_observation(
                &format!("roundtrip-{repetition}"),
                "parse-format-reparse",
                repetition,
                &format!("{}", repetition * 100),
                (repetition == 3).then_some("interrupted by controlled fixture"),
            )
            .expect("roundtrip"),
        );
        values.push(
            measurement_observation(
                &format!("complete-{repetition}"),
                "complete-run",
                repetition,
                &format!("{}", repetition * 1000),
                None,
            )
            .expect("complete"),
        );
    }
    values
}

fn series() -> Value {
    record_measurement_series(CANDIDATE, ENVIRONMENT, &observations(), &context())
        .expect("valid series")
}

fn assert_invalid(series: &Value) {
    assert_eq!(
        validate_measurement_series(series, CANDIDATE, ENVIRONMENT)
            .expect_err("invalid series")
            .code(),
        VerificationErrorCode::InvalidMeasurementSeries
    );
}

#[trace("TC-036", "FR-020-AC-1")]
#[trace("TC-036", "FR-020-AC-2")]
#[trace("TC-036", "FR-020-AC-3")]
#[trace("TC-036", "FR-020-AC-4")]
#[trace("TC-036", "FR-020-AC-5")]
#[test]
fn tc_036_retains_exact_sequence_and_recomputable_lower_medians() {
    let series = series();
    let _: wire::Vp04MeasurementSeries =
        serde_json::from_value(series.clone()).expect("schema-generated measurement type");
    assert_eq!(
        series["observations"]
            .as_array()
            .expect("observations")
            .len(),
        16
    );
    assert_eq!(series["observations"][0]["phase"], "setup");
    assert_eq!(series["observations"][0]["repetition"], 0);
    assert_eq!(series["observations"][8]["status"], "invalid");
    assert_eq!(series["observations"][8]["rejectedElapsed"], "300");
    assert!(
        series["observations"][8]
            .get("elapsedNanoseconds")
            .is_none()
    );
    assert_eq!(series["summaries"][0]["validSampleCount"], 1);
    assert_eq!(series["summaries"][1]["medianNanoseconds"], 30);
    assert_eq!(series["summaries"][2]["validSampleCount"], 4);
    assert_eq!(series["summaries"][2]["medianNanoseconds"], 200);
    assert_eq!(series["summaries"][3]["maximumNanoseconds"], 5000);
    validate_measurement_series(&series, CANDIDATE, ENVIRONMENT).expect("exact recomputation");

    let mut changed = series.clone();
    changed["summaries"][1]["medianNanoseconds"] = json!(31);
    assert_invalid(&changed);
    let mut changed = series.clone();
    changed["environmentIdentity"] = json!("another environment");
    assert_invalid(&changed);
}

#[trace("TC-036", "FR-020-AC-3", "FR-020-AC-8")]
#[test]
fn tc_036_classifies_duration_domain_without_rewriting_rejected_scalars() {
    for (raw, admitted) in [
        ("0", true),
        (&MAX_ELAPSED_NANOSECONDS.to_string(), true),
        ("-1", false),
        ("1.5", false),
        ("9007199254740992", false),
    ] {
        let observation = measurement_observation("sample", "setup", 0, raw, None)
            .expect("classified observation");
        if admitted {
            assert_eq!(observation["status"], "valid");
            assert!(observation.get("rejectedElapsed").is_none());
        } else {
            assert_eq!(observation["status"], "invalid");
            assert_eq!(observation["rejectedElapsed"], raw);
            assert!(observation.get("elapsedNanoseconds").is_none());
        }
    }
    assert_eq!(
        measurement_observation("sample", "setup", 0, &"1".repeat(65), None)
            .expect_err("unbounded elapsed scalar")
            .code(),
        VerificationErrorCode::InvalidMeasurementSeries
    );
}

#[trace("TC-036", "FR-020-AC-2", "FR-020-AC-3", "FR-020-AC-9")]
#[test]
fn tc_036_refuses_identity_order_pair_and_tagged_variant_drift() {
    let baseline = series();
    let mut cases = Vec::new();

    let mut changed = baseline.clone();
    changed["observations"][0]["observationId"] = json!("");
    cases.push(changed);
    let mut changed = baseline.clone();
    changed["observations"][1]["observationId"] =
        changed["observations"][0]["observationId"].clone();
    cases.push(changed);
    let mut changed = baseline.clone();
    changed["observations"]
        .as_array_mut()
        .expect("observations")
        .swap(1, 2);
    cases.push(changed);
    let mut changed = baseline.clone();
    changed["observations"]
        .as_array_mut()
        .expect("observations")
        .pop();
    cases.push(changed);
    let mut changed = baseline.clone();
    changed["observations"][1]["repetition"] = json!(0);
    cases.push(changed);
    let mut changed = baseline.clone();
    changed["observations"][1]["repetition"] = json!(6);
    cases.push(changed);
    let mut changed = baseline.clone();
    changed["observations"][1]["reason"] = json!("invalid-only field");
    cases.push(changed);
    let mut changed = baseline.clone();
    changed["observations"][8]["elapsedNanoseconds"] = json!(300);
    cases.push(changed);
    let mut changed = baseline.clone();
    changed["candidateIdentity"] = json!("");
    cases.push(changed);
    let mut changed = baseline.clone();
    changed["context"]["commandOrder"] = json!(["complete-run", "setup"]);
    cases.push(changed);

    for changed in cases {
        assert_invalid(&changed);
    }
}
