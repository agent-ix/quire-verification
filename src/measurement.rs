// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! VP04's fixed parser-portfolio measurement series and summaries.
//!
//! Governing requirement: FR-020.

use std::collections::BTreeSet;

use serde_json::{Value, json};

use crate::contracts::{VerificationError, VerificationErrorCode, validate_schema_definition};

/// Largest integer duration admitted by the JSON interchange contract.
pub const MAX_ELAPSED_NANOSECONDS: u64 = 9_007_199_254_740_991;

const PHASES: [&str; 4] = [
    "setup",
    "direct-parse",
    "parse-format-reparse",
    "complete-run",
];

/// Builds a tagged observation while preserving rejected JSON scalar spelling.
///
/// Supplying a reason always produces an invalid observation. Without a reason,
/// only an integer in the admitted interchange range becomes valid.
///
/// # Errors
/// Returns `invalid_measurement_series` when the scalar is empty or is not one
/// exact JSON scalar value.
pub fn measurement_observation(
    observation_id: &str,
    phase: &str,
    repetition: u64,
    elapsed_json_scalar: &str,
    invalid_reason: Option<&str>,
) -> Result<Value, VerificationError> {
    if elapsed_json_scalar.is_empty() || elapsed_json_scalar.len() > 64 {
        return Err(invalid_series("elapsed scalar is empty or unbounded"));
    }
    let parsed: Value = serde_json::from_str(elapsed_json_scalar)
        .map_err(|_| invalid_series("elapsed value is not exactly one JSON scalar"))?;
    if parsed.is_array() || parsed.is_object() {
        return Err(invalid_series("elapsed value is not a JSON scalar"));
    }
    if let Some(reason) = invalid_reason {
        return invalid_observation(
            observation_id,
            phase,
            repetition,
            reason,
            elapsed_json_scalar,
        );
    }
    if let Some(elapsed) = parsed
        .as_u64()
        .filter(|value| *value <= MAX_ELAPSED_NANOSECONDS)
    {
        let observation = json!({
            "observationId": observation_id,
            "phase": phase,
            "repetition": repetition,
            "status": "valid",
            "elapsedNanoseconds": elapsed,
        });
        validate_schema_definition("Vp04MeasurementObservation", &observation)
            .map_err(|_| invalid_series("observation identity or phase is invalid"))?;
        return Ok(observation);
    }
    invalid_observation(
        observation_id,
        phase,
        repetition,
        "elapsed duration outside admitted integer domain",
        elapsed_json_scalar,
    )
}

/// Builds and semantically validates the fixed VP04 measurement series.
///
/// # Errors
/// Returns `invalid_measurement_series` for identity, shape, order, tagged
/// variant, or summary violations.
pub fn record_measurement_series(
    candidate_identity: &str,
    environment_identity: &str,
    observations: &[Value],
    context: &Value,
) -> Result<Value, VerificationError> {
    if candidate_identity.is_empty() || environment_identity.is_empty() {
        return Err(invalid_series("series identity is empty"));
    }
    validate_observation_sequence(observations)?;
    let summaries = recompute_summaries(observations)?;
    let series = json!({
        "schemaVersion": "quire-verification.vp04-measurements/v1",
        "candidateIdentity": candidate_identity,
        "environmentIdentity": environment_identity,
        "observations": observations,
        "summaries": summaries,
        "context": context,
    });
    validate_measurement_series(&series, candidate_identity, environment_identity)?;
    Ok(series)
}

/// Validates a retained series against its selected identities and recomputed summaries.
///
/// # Errors
/// Returns `invalid_measurement_series` for every schema or semantic difference.
pub fn validate_measurement_series(
    series: &Value,
    expected_candidate_identity: &str,
    expected_environment_identity: &str,
) -> Result<(), VerificationError> {
    validate_schema_definition("Vp04MeasurementSeries", series)
        .map_err(|_| invalid_series("measurement series violates its wire schema"))?;
    if series["candidateIdentity"] != expected_candidate_identity
        || series["environmentIdentity"] != expected_environment_identity
    {
        return Err(invalid_series("measurement series identity differs"));
    }
    let observations = series["observations"]
        .as_array()
        .ok_or_else(|| invalid_series("measurement observations are absent"))?;
    validate_observation_sequence(observations)?;
    let expected = recompute_summaries(observations)?;
    if series["summaries"] != expected {
        return Err(invalid_series(
            "measurement summaries do not recompute exactly",
        ));
    }
    Ok(())
}

/// Recomputes phase summaries using valid samples and the lower-median rule.
///
/// # Errors
/// Returns `invalid_measurement_series` for malformed observations.
fn recompute_summaries(observations: &[Value]) -> Result<Value, VerificationError> {
    let mut summaries = Vec::with_capacity(PHASES.len());
    for phase in PHASES {
        let mut values = observations
            .iter()
            .filter(|observation| observation["phase"] == phase && observation["status"] == "valid")
            .map(|observation| {
                observation["elapsedNanoseconds"]
                    .as_u64()
                    .filter(|value| *value <= MAX_ELAPSED_NANOSECONDS)
                    .ok_or_else(|| invalid_series("valid elapsed duration is outside its domain"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        values.sort_unstable();
        let (minimum, median, maximum) = if values.is_empty() {
            (Value::Null, Value::Null, Value::Null)
        } else {
            (
                json!(values[0]),
                json!(values[(values.len() - 1) / 2]),
                json!(values[values.len() - 1]),
            )
        };
        summaries.push(json!({
            "phase": phase,
            "validSampleCount": values.len(),
            "minimumNanoseconds": minimum,
            "medianNanoseconds": median,
            "maximumNanoseconds": maximum,
        }));
    }
    Ok(Value::Array(summaries))
}

fn invalid_observation(
    observation_id: &str,
    phase: &str,
    repetition: u64,
    reason: &str,
    rejected_elapsed: &str,
) -> Result<Value, VerificationError> {
    if reason.is_empty() {
        return Err(invalid_series("invalid observation reason is empty"));
    }
    let observation = json!({
        "observationId": observation_id,
        "phase": phase,
        "repetition": repetition,
        "status": "invalid",
        "reason": reason,
        "rejectedElapsed": rejected_elapsed,
    });
    validate_schema_definition("Vp04MeasurementObservation", &observation)
        .map_err(|_| invalid_series("observation identity or phase is invalid"))?;
    Ok(observation)
}

fn validate_observation_sequence(observations: &[Value]) -> Result<(), VerificationError> {
    if observations.len() != 16 {
        return Err(invalid_series(
            "measurement series must contain 16 observations",
        ));
    }
    let mut expected = vec![("setup", 0_u64)];
    for repetition in 1..=5 {
        expected.extend([
            ("direct-parse", repetition),
            ("parse-format-reparse", repetition),
            ("complete-run", repetition),
        ]);
    }
    let mut ids = BTreeSet::new();
    let mut pairs = BTreeSet::new();
    for (observation, (expected_phase, expected_repetition)) in observations.iter().zip(expected) {
        validate_schema_definition("Vp04MeasurementObservation", observation)
            .map_err(|_| invalid_series("measurement observation violates its tagged schema"))?;
        let id = observation["observationId"]
            .as_str()
            .ok_or_else(|| invalid_series("observation ID is absent"))?;
        let phase = observation["phase"]
            .as_str()
            .ok_or_else(|| invalid_series("observation phase is absent"))?;
        let repetition = observation["repetition"]
            .as_u64()
            .ok_or_else(|| invalid_series("observation repetition is absent"))?;
        if !ids.insert(id) || !pairs.insert((phase, repetition)) {
            return Err(invalid_series("measurement identity is duplicated"));
        }
        if phase != expected_phase || repetition != expected_repetition {
            return Err(invalid_series(
                "measurement observations are not in prescribed order",
            ));
        }
    }
    Ok(())
}

fn invalid_series(message: impl Into<String>) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::InvalidMeasurementSeries,
        message.into(),
    )
}
