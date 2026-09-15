// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Property tests generated from the extractable FR-030 obligations.

use std::collections::BTreeMap;

use ix_trace_rs::trace;
use proptest::prelude::*;
use quire_verification::relational_hyperproperty::{
    AlignedPosition, AlignmentKind, ComparisonOperator, OracleDependence, QuantifiedTrace,
    Quantifier, RELATIONAL_REQUEST_VERSION, RelationalExpression, RelationalMethodOutcome,
    RelationalRequest, RelationalValue, Trace, TraceAlignment, TraceIdentity, TracePoint,
    TracePopulation, TraceValueRef, evaluate_relational,
};

fn trace_fixture(identity: &str, local_name: &str, value: i64) -> Trace {
    Trace {
        identity: TraceIdentity::new(identity).expect("static fixture identity"),
        local_name: local_name.to_owned(),
        attributes: BTreeMap::new(),
        points: vec![TracePoint {
            event_identity: format!("event:{identity}:0"),
            values: BTreeMap::from([("value".to_owned(), RelationalValue::Integer(value))]),
        }],
    }
}

fn request(first: Quantifier, second: Quantifier, low: i64, high: i64) -> RelationalRequest {
    RelationalRequest {
        contract_version: RELATIONAL_REQUEST_VERSION.to_owned(),
        provider_identity: "provider:relational-property@1".to_owned(),
        semantic_subject_identity: "subject:relational-property".to_owned(),
        finite_assignment_bound: 16,
        populations: vec![TracePopulation {
            identity: "population:property".to_owned(),
            closure_identity: Some("closure:property@1".to_owned()),
            traces: vec![
                trace_fixture("trace:low", "same-local-name", low),
                trace_fixture("trace:high", "same-local-name", high),
            ],
        }],
        prefix: vec![
            QuantifiedTrace {
                variable: "x".to_owned(),
                quantifier: first,
                population_identity: "population:property".to_owned(),
            },
            QuantifiedTrace {
                variable: "y".to_owned(),
                quantifier: second,
                population_identity: "population:property".to_owned(),
            },
        ],
        expression: RelationalExpression::Compare {
            left: TraceValueRef {
                variable: "x".to_owned(),
                field: "value".to_owned(),
            },
            operator: ComparisonOperator::LessThan,
            right: TraceValueRef {
                variable: "y".to_owned(),
                field: "value".to_owned(),
            },
        },
        alignments: vec![TraceAlignment {
            identity: "alignment:low-high".to_owned(),
            left_trace: TraceIdentity::new("trace:low").expect("static fixture identity"),
            right_trace: TraceIdentity::new("trace:high").expect("static fixture identity"),
            kind: AlignmentKind::ExplicitPosition,
            pairs: vec![AlignedPosition { left: 0, right: 0 }],
        }],
        assumptions: vec![],
        oracle_dependence: OracleDependence::Independent,
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// Tracing: TC-048, FR-030-AC-2, IT-005-SC-02.
    /// spec-correctness: row=FR-030-AC-2 property=quantifier-order extraction=extractable origin=regex
    #[trace("TC-048", "FR-030-AC-2", "IT-005")]
    #[test]
    fn prop_fr_030_quantifier_order_is_semantic_and_not_local_name_based(
        low in -100_i64..100,
        gap in 1_i64..32,
    ) {
        let high = low + gap;
        let all_then_exists = request(Quantifier::ForAll, Quantifier::Exists, low, high);
        let exists_then_all = request(Quantifier::Exists, Quantifier::ForAll, low, high);

        prop_assert!(matches!(evaluate_relational(&all_then_exists), RelationalMethodOutcome::Refuted(_)));
        prop_assert!(matches!(evaluate_relational(&exists_then_all), RelationalMethodOutcome::Proved(_)));
    }
}
