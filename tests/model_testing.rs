// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! TC-047 and IT-005-SC-01 model generation and shrinking coverage.

use std::collections::BTreeMap;

use ix_trace_rs::trace;
use quire_verification::model_testing::{
    AdmittedCase, CheckedDomain, CheckedField, CheckedModel, CheckedPredicate, FailureSignature,
    MODEL_TEST_REQUEST_VERSION, ModelCase, ModelFailureCode, ModelMethodOutcome, ModelRefusalCode,
    ModelTestRequest, ModelValue, NativeReplay, ReplayDisposition, ReplayObservation,
    ShrinkRejection, generate_model_cases, run_model_tests, shrink_counterexample,
};

fn value(integer: i64) -> ModelValue {
    ModelValue::Integer(integer)
}

fn assignment(entries: &[(&str, ModelValue)]) -> BTreeMap<String, ModelValue> {
    entries
        .iter()
        .map(|(field, value)| ((*field).to_owned(), value.clone()))
        .collect()
}

fn checked_model() -> CheckedModel {
    CheckedModel {
        package_identity: "package:checked@abc".to_owned(),
        profile_identity: "profile:model-testing/v1".to_owned(),
        semantic_subject_identity: "subject:counter".to_owned(),
        native_runtime_identity: "runtime:native@def".to_owned(),
        ecosystem_lock_identity: "lock:ecosystem@123".to_owned(),
        fields: vec![
            CheckedField {
                id: "low".to_owned(),
                domain: CheckedDomain::Integer {
                    minimum: 0,
                    maximum: 2,
                },
            },
            CheckedField {
                id: "high".to_owned(),
                domain: CheckedDomain::Integer {
                    minimum: 0,
                    maximum: 2,
                },
            },
            CheckedField {
                id: "items".to_owned(),
                domain: CheckedDomain::Collection {
                    minimum_items: 0,
                    maximum_items: 1,
                    element: Box::new(CheckedDomain::Enumeration {
                        values: vec!["only".to_owned()],
                    }),
                },
            },
        ],
        preconditions: vec![],
        relationships: vec![CheckedPredicate::LessThanOrEqual {
            left: "low".to_owned(),
            right: "high".to_owned(),
        }],
        construction_bound: 128,
    }
}

#[derive(Default)]
struct ThresholdReplay {
    seen: Vec<BTreeMap<String, ModelValue>>,
}

impl NativeReplay for ThresholdReplay {
    fn replay(&mut self, case: &AdmittedCase) -> ReplayObservation {
        self.seen.push(case.values.clone());
        let high = match case.values.get("high") {
            Some(ModelValue::Integer(value)) => *value,
            _ => {
                return ReplayObservation::Failed {
                    code: "fixture-shape".to_owned(),
                    message: "high is not an integer".to_owned(),
                };
            }
        };
        if high == 2 {
            ReplayObservation::Refuted(FailureSignature {
                clause_id: "clause:upper-bound".to_owned(),
                disposition: ReplayDisposition::Refuted,
            })
        } else {
            ReplayObservation::Satisfied {
                clause_id: "clause:upper-bound".to_owned(),
            }
        }
    }
}

/// Tracing: TC-047, FR-029-AC-1, IT-005-SC-01.
#[trace("TC-047", "FR-029-AC-1", "IT-005")]
#[test]
fn tc_047_generates_complete_deterministic_boundary_partitions() {
    let request = ModelTestRequest {
        contract_version: MODEL_TEST_REQUEST_VERSION.to_owned(),
        model: checked_model(),
        seed: 0x51_17,
        case_bound: 128,
        shrink_bound: 128,
    };

    let first = generate_model_cases(&request).expect("checked domain is constructible");
    let second = generate_model_cases(&request).expect("same checked domain is constructible");
    assert_eq!(first.identity, second.identity);
    assert_eq!(first.cases, second.cases);

    let admitted = first
        .cases
        .iter()
        .filter_map(|case| match case {
            ModelCase::Admitted(case) => Some(case),
            ModelCase::RefusalProbe(_) => None,
        })
        .collect::<Vec<_>>();
    assert!(
        admitted
            .iter()
            .all(|case| request.model.admits(&case.values))
    );
    assert!(
        admitted
            .iter()
            .any(|case| case.partitions.contains(&"items:empty".to_owned()))
    );
    assert!(
        admitted
            .iter()
            .any(|case| case.partitions.contains(&"items:singleton".to_owned()))
    );
    assert!(
        admitted
            .iter()
            .any(|case| case.partitions.contains(&"low:minimum".to_owned()))
    );
    assert!(
        admitted
            .iter()
            .any(|case| case.partitions.contains(&"high:maximum".to_owned()))
    );
    assert!(admitted.iter().any(|case| {
        case.partitions
            .iter()
            .any(|p| p.starts_with("relationship:0:boundary"))
    }));

    let refusal_partitions = first
        .cases
        .iter()
        .filter_map(|case| match case {
            ModelCase::RefusalProbe(probe) => Some(probe.partition.as_str()),
            ModelCase::Admitted(_) => None,
        })
        .collect::<Vec<_>>();
    assert!(refusal_partitions.contains(&"low:just-below"));
    assert!(refusal_partitions.contains(&"high:just-above"));
    assert!(refusal_partitions.contains(&"items:just-above"));
}

/// Tracing: TC-047, FR-029-AC-2, IT-005-SC-01.
#[trace("TC-047", "FR-029-AC-2", "IT-005")]
#[test]
fn tc_047_retains_only_strict_domain_valid_same_failure_shrinks() {
    let model = checked_model();
    let original = AdmittedCase::new(
        assignment(&[
            ("low", value(2)),
            ("high", value(2)),
            (
                "items",
                ModelValue::Collection(vec![ModelValue::Label("only".to_owned())]),
            ),
        ]),
        vec!["fixture:original".to_owned()],
        &model,
    )
    .expect("original is admitted");
    let equal = original.clone();
    let outside = assignment(&[
        ("low", value(-1)),
        ("high", value(2)),
        ("items", ModelValue::Collection(vec![])),
    ]);
    let changed_verdict = assignment(&[
        ("low", value(0)),
        ("high", value(1)),
        ("items", ModelValue::Collection(vec![])),
    ]);
    let valid_same_failure = assignment(&[
        ("low", value(0)),
        ("high", value(2)),
        ("items", ModelValue::Collection(vec![])),
    ]);
    let signature = FailureSignature {
        clause_id: "clause:upper-bound".to_owned(),
        disposition: ReplayDisposition::Refuted,
    };
    let mut replay = ThresholdReplay::default();

    let result = shrink_counterexample(
        &model,
        &original,
        &[equal.values, outside, changed_verdict, valid_same_failure],
        &signature,
        16,
        &mut replay,
    )
    .expect("valid shrink request");

    assert_eq!(result.retained.len(), 1);
    assert_eq!(result.retained_replays.len(), result.retained.len());
    assert_eq!(
        result.retained_replays[0].case_identity,
        result.retained[0].identity
    );
    assert!(model.strictly_smaller(&result.retained[0].values, &original.values));
    assert!(result.complete);
    assert_eq!(result.smallest_retained, result.retained[0]);
    assert_eq!(
        result.minimal_counterexample,
        Some(result.retained[0].clone())
    );
    assert!(
        result
            .rejected
            .iter()
            .any(|item| item.reason == ShrinkRejection::NotStrictlySmaller)
    );
    assert!(
        result
            .rejected
            .iter()
            .any(|item| item.reason == ShrinkRejection::OutsideDomain)
    );
    assert!(
        result
            .rejected
            .iter()
            .any(|item| item.reason == ShrinkRejection::ChangedFailure)
    );

    let bounded = shrink_counterexample(
        &model,
        &original,
        &[result.retained[0].values.clone()],
        &signature,
        0,
        &mut replay,
    )
    .expect("zero is an admitted explicit work bound for direct shrink inspection");
    assert!(!bounded.complete);
    assert!(bounded.minimal_counterexample.is_none());
    assert_eq!(bounded.smallest_retained, original);
}

struct RejectingReplay;

impl NativeReplay for RejectingReplay {
    fn replay(&mut self, _case: &AdmittedCase) -> ReplayObservation {
        ReplayObservation::RejectedPrecondition {
            precondition_id: "native:unexpected-precondition".to_owned(),
        }
    }
}

#[derive(Default)]
struct SatisfyingReplay {
    seen: Vec<String>,
}

impl NativeReplay for SatisfyingReplay {
    fn replay(&mut self, case: &AdmittedCase) -> ReplayObservation {
        self.seen.push(case.identity.clone());
        ReplayObservation::Satisfied {
            clause_id: "clause:all-cases".to_owned(),
        }
    }
}

struct FailedReplay;

impl NativeReplay for FailedReplay {
    fn replay(&mut self, _case: &AdmittedCase) -> ReplayObservation {
        ReplayObservation::Failed {
            code: "native-crash".to_owned(),
            message: "native provider exited without a result".to_owned(),
        }
    }
}

/// Tracing: TC-047, FR-029-AC-3, IT-005-SC-01.
#[trace("TC-047", "FR-029-AC-3", "IT-005")]
#[test]
fn tc_047_refuses_unsatisfiable_domains_and_never_passes_rejected_preconditions() {
    let mut model = checked_model();
    model.preconditions = vec![
        CheckedPredicate::LessThan {
            left: "high".to_owned(),
            right: "low".to_owned(),
        },
        CheckedPredicate::LessThanOrEqual {
            left: "low".to_owned(),
            right: "high".to_owned(),
        },
    ];
    let unsatisfiable = ModelTestRequest {
        contract_version: MODEL_TEST_REQUEST_VERSION.to_owned(),
        model,
        seed: 7,
        case_bound: 64,
        shrink_bound: 64,
    };
    assert!(matches!(
        run_model_tests(&unsatisfiable, &mut RejectingReplay),
        ModelMethodOutcome::Refused(refusal) if refusal.code == ModelRefusalCode::UnsatisfiableDomain
    ));

    let admitted = ModelTestRequest {
        contract_version: MODEL_TEST_REQUEST_VERSION.to_owned(),
        model: checked_model(),
        seed: 7,
        case_bound: 64,
        shrink_bound: 64,
    };
    assert!(matches!(
        run_model_tests(&admitted, &mut RejectingReplay),
        ModelMethodOutcome::Failed(failure) if failure.code == ModelFailureCode::RejectedPrecondition
    ));

    let mut unsupported = admitted.clone();
    unsupported.contract_version = "quire.model-test-request/v2".to_owned();
    assert!(matches!(
        run_model_tests(&unsupported, &mut SatisfyingReplay::default()),
        ModelMethodOutcome::Refused(refusal) if refusal.code == ModelRefusalCode::InvalidDomain
    ));

    let mut oversized = admitted;
    oversized.model.fields = vec![CheckedField {
        id: "items".to_owned(),
        domain: CheckedDomain::Collection {
            minimum_items: 129,
            maximum_items: 129,
            element: Box::new(CheckedDomain::Boolean),
        },
    }];
    oversized.model.relationships.clear();
    assert!(matches!(
        run_model_tests(&oversized, &mut SatisfyingReplay::default()),
        ModelMethodOutcome::Refused(refusal)
            if refusal.code == ModelRefusalCode::ConstructionBoundExceeded
    ));
}

/// Tracing: TC-047, FR-029-AC-3.
#[trace("TC-047", "FR-029-AC-3")]
#[test]
fn tc_047_run_identity_binds_seed_bounds_and_exact_provider_inputs() {
    let request = ModelTestRequest {
        contract_version: MODEL_TEST_REQUEST_VERSION.to_owned(),
        model: checked_model(),
        seed: 41,
        case_bound: 64,
        shrink_bound: 64,
    };
    let mut first_replay = ThresholdReplay::default();
    let mut second_replay = ThresholdReplay::default();
    let first = run_model_tests(&request, &mut first_replay);
    let second = run_model_tests(&request, &mut second_replay);
    assert_eq!(first.identity(), second.identity());
    assert_eq!(first, second);

    let mut changed = request.clone();
    changed.seed += 1;
    let changed = run_model_tests(&changed, &mut ThresholdReplay::default());
    assert_ne!(first.identity(), changed.identity());

    let mut changed_profile = request.clone();
    changed_profile.model.profile_identity = "profile:model-testing/v2".to_owned();
    let changed_profile = run_model_tests(&changed_profile, &mut ThresholdReplay::default());
    assert_ne!(first.identity(), changed_profile.identity());
}

/// Tracing: TC-047, FR-029-AC-1, FR-029-AC-3, IT-005-SC-01.
#[trace("TC-047", "FR-029-AC-1", "FR-029-AC-3", "IT-005")]
#[test]
fn tc_047_executes_only_admitted_cases_and_keeps_native_failure_nonsemantic() {
    let request = ModelTestRequest {
        contract_version: MODEL_TEST_REQUEST_VERSION.to_owned(),
        model: checked_model(),
        seed: 91,
        case_bound: 128,
        shrink_bound: 128,
    };
    let generated = generate_model_cases(&request).expect("constructible model");
    let admitted_count = generated
        .cases
        .iter()
        .filter(|case| matches!(case, ModelCase::Admitted(_)))
        .count();

    let mut satisfying = SatisfyingReplay::default();
    let outcome = run_model_tests(&request, &mut satisfying);
    let ModelMethodOutcome::Tested(report) = outcome else {
        panic!("all-satisfying native replay must complete as tested");
    };
    assert_eq!(satisfying.seen.len(), admitted_count);
    assert_eq!(report.observations.len(), admitted_count);
    assert!(report.counterexample.is_none());

    assert!(matches!(
        run_model_tests(&request, &mut FailedReplay),
        ModelMethodOutcome::Failed(failure)
            if failure.code == ModelFailureCode::NativeReplayFailed
                && failure.provider_code.as_deref() == Some("native-crash")
    ));
}
