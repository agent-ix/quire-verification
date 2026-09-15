// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Property tests generated from the extractable FR-029 obligations.

use ix_trace_rs::trace;
use proptest::prelude::*;
use quire_verification::model_testing::{
    CheckedDomain, CheckedField, CheckedModel, CheckedPredicate, MODEL_TEST_REQUEST_VERSION,
    ModelCase, ModelTestRequest, generate_model_cases,
};

fn request(seed: u64, minimum: i64, maximum: i64) -> ModelTestRequest {
    ModelTestRequest {
        contract_version: MODEL_TEST_REQUEST_VERSION.to_owned(),
        model: CheckedModel {
            package_identity: "package:property-fixture@1".to_owned(),
            profile_identity: "profile:model-testing/v1".to_owned(),
            semantic_subject_identity: "subject:property-fixture".to_owned(),
            native_runtime_identity: "runtime:native@1".to_owned(),
            ecosystem_lock_identity: "lock:property-fixture@1".to_owned(),
            fields: vec![
                CheckedField {
                    id: "left".to_owned(),
                    domain: CheckedDomain::Integer { minimum, maximum },
                },
                CheckedField {
                    id: "right".to_owned(),
                    domain: CheckedDomain::Integer { minimum, maximum },
                },
            ],
            preconditions: vec![],
            relationships: vec![CheckedPredicate::LessThanOrEqual {
                left: "left".to_owned(),
                right: "right".to_owned(),
            }],
            construction_bound: 256,
        },
        seed,
        case_bound: 260,
        shrink_bound: 256,
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// Tracing: TC-047, FR-029-AC-1, FR-029-AC-3.
    /// spec-correctness: row=FR-029-AC-1 property=universal extraction=extractable origin=regex
    #[trace("TC-047", "FR-029-AC-1", "FR-029-AC-3")]
    #[test]
    fn prop_fr_029_ac_1_and_3_are_domain_valid_and_seed_deterministic(
        seed in any::<u64>(),
        minimum in -8_i64..=8,
        width in 0_i64..=5,
    ) {
        let maximum = minimum + width;
        let request = request(seed, minimum, maximum);
        let first = generate_model_cases(&request).expect("bounded generated domain");
        let second = generate_model_cases(&request).expect("same bounded generated domain");
        prop_assert_eq!(&first, &second);

        for case in &first.cases {
            if let ModelCase::Admitted(case) = case {
                prop_assert!(request.model.admits(&case.values));
            }
        }
        let partitions = first.cases.iter().flat_map(|case| match case {
            ModelCase::Admitted(case) => case.partitions.iter().map(String::as_str).collect(),
            ModelCase::RefusalProbe(probe) => vec![probe.partition.as_str()],
        }).collect::<Vec<_>>();
        prop_assert!(partitions.contains(&"left:minimum"));
        prop_assert!(partitions.contains(&"right:maximum"));
        prop_assert!(partitions.contains(&"relationship:0:boundary"));
        if minimum > i64::MIN {
            prop_assert!(partitions.contains(&"left:just-below"));
        }
        if maximum < i64::MAX {
            prop_assert!(partitions.contains(&"right:just-above"));
        }
    }
}
