// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! TC-048 and IT-005-SC-02 relational/hyperproperty coverage.

use std::collections::BTreeMap;

use ix_trace_rs::trace;
use quire_verification::relational_hyperproperty::{
    AlignedPosition, AlignmentKind, ComparisonOperator, OracleDependence, PopulationAssumption,
    QuantifiedTrace, Quantifier, RELATIONAL_REQUEST_VERSION, RelationalExpression,
    RelationalMethodOutcome, RelationalNonConclusionCode, RelationalRequest, RelationalValue,
    Trace, TraceAlignment, TraceAssumptionKind, TraceIdentity, TracePoint, TracePopulation,
    TraceValueRef,
};

fn trace(identity: &str, local_name: &str, value: i64, class: &str) -> Trace {
    Trace {
        identity: TraceIdentity::new(identity).expect("fixture trace identity"),
        local_name: local_name.to_owned(),
        attributes: BTreeMap::from([(
            "class".to_owned(),
            RelationalValue::Label(class.to_owned()),
        )]),
        points: vec![TracePoint {
            event_identity: format!("event:{identity}:0"),
            values: BTreeMap::from([("value".to_owned(), RelationalValue::Integer(value))]),
        }],
    }
}

fn alignment(left: &str, right: &str) -> TraceAlignment {
    TraceAlignment {
        identity: format!("alignment:{left}:{right}"),
        left_trace: TraceIdentity::new(left).expect("left identity"),
        right_trace: TraceIdentity::new(right).expect("right identity"),
        kind: AlignmentKind::ExplicitPosition,
        pairs: vec![AlignedPosition { left: 0, right: 0 }],
    }
}

fn prefix(first: Quantifier, second: Quantifier) -> Vec<QuantifiedTrace> {
    vec![
        QuantifiedTrace {
            variable: "x".to_owned(),
            quantifier: first,
            population_identity: "population:main".to_owned(),
        },
        QuantifiedTrace {
            variable: "y".to_owned(),
            quantifier: second,
            population_identity: "population:main".to_owned(),
        },
    ]
}

fn expression() -> RelationalExpression {
    RelationalExpression::Compare {
        left: TraceValueRef {
            variable: "x".to_owned(),
            field: "value".to_owned(),
        },
        operator: ComparisonOperator::LessThan,
        right: TraceValueRef {
            variable: "y".to_owned(),
            field: "value".to_owned(),
        },
    }
}

fn request(first: Quantifier, second: Quantifier) -> RelationalRequest {
    RelationalRequest {
        contract_version: RELATIONAL_REQUEST_VERSION.to_owned(),
        provider_identity: "provider:relational-rust@1".to_owned(),
        semantic_subject_identity: "subject:checked-traces@abc".to_owned(),
        finite_assignment_bound: 16,
        populations: vec![TracePopulation {
            identity: "population:main".to_owned(),
            closure_identity: Some("closure:main@1".to_owned()),
            traces: vec![
                trace("trace:a", "same-local-name", 0, "public"),
                trace("trace:b", "same-local-name", 1, "public"),
            ],
        }],
        prefix: prefix(first, second),
        expression: expression(),
        alignments: vec![alignment("trace:a", "trace:b")],
        assumptions: vec![],
        oracle_dependence: OracleDependence::Shared {
            group_identity: "oracle-group:checked-runtime".to_owned(),
        },
    }
}

/// Tracing: TC-048, FR-030-AC-2, IT-005-SC-02.
#[trace("TC-048", "FR-030-AC-2", "IT-005")]
#[test]
fn tc_048_preserves_mixed_quantifier_order_and_changes_the_result() {
    let universal_exists = quire_verification::relational_hyperproperty::evaluate_relational(
        &request(Quantifier::ForAll, Quantifier::Exists),
    );
    let exists_universal = quire_verification::relational_hyperproperty::evaluate_relational(
        &request(Quantifier::Exists, Quantifier::ForAll),
    );

    assert!(matches!(
        universal_exists,
        RelationalMethodOutcome::Refuted(_)
    ));
    assert!(matches!(
        exists_universal,
        RelationalMethodOutcome::Proved(_)
    ));
    assert_ne!(universal_exists.identity(), exists_universal.identity());
}

/// Tracing: TC-048, FR-030-AC-1, IT-005-SC-02.
#[trace("TC-048", "FR-030-AC-1", "IT-005")]
#[test]
fn tc_048_returns_exact_paired_witnesses_and_identity_bound_assumptions() {
    let base = request(Quantifier::Exists, Quantifier::ForAll);
    let outcome = quire_verification::relational_hyperproperty::evaluate_relational(&base);
    let RelationalMethodOutcome::Proved(result) = outcome else {
        panic!("fixture must be proved");
    };
    assert_eq!(result.witness.decisive_assignment["x"].as_str(), "trace:a");
    assert_eq!(result.witness.decisive_assignment["y"].as_str(), "trace:b");
    assert_eq!(
        result.witness.alignment_identities,
        vec!["alignment:trace:a:trace:b"]
    );
    assert_eq!(result.witness.evaluated_assignments.len(), 1);
    assert_eq!(result.oracle_dependence, base.oracle_dependence);

    let mut assumed = base.clone();
    assumed.populations[0]
        .traces
        .push(trace("trace:c", "third", 2, "secret"));
    assumed.alignments.extend([
        alignment("trace:a", "trace:c"),
        alignment("trace:b", "trace:c"),
    ]);
    assumed.assumptions.push(PopulationAssumption {
        identity: "assumption:public-only".to_owned(),
        kind: TraceAssumptionKind::Environment,
        population_identity: "population:main".to_owned(),
        attribute: "class".to_owned(),
        equals: RelationalValue::Label("public".to_owned()),
    });
    assumed.assumptions.push(PopulationAssumption {
        identity: "assumption:declassified-public".to_owned(),
        kind: TraceAssumptionKind::Declassification,
        population_identity: "population:main".to_owned(),
        attribute: "class".to_owned(),
        equals: RelationalValue::Label("public".to_owned()),
    });
    let assumed_outcome =
        quire_verification::relational_hyperproperty::evaluate_relational(&assumed);
    assert!(matches!(
        assumed_outcome,
        RelationalMethodOutcome::Proved(_)
    ));
    assert_ne!(result.identity, assumed_outcome.identity());
}

/// Tracing: TC-048, FR-030-AC-3, FR-030-CON-1, IT-005-SC-02.
#[trace("TC-048", "FR-030-AC-3", "FR-030-CON-1", "IT-005")]
#[test]
fn tc_048_keeps_missing_closure_alignment_and_bounds_nonconclusive() {
    let mut unclosed = request(Quantifier::Exists, Quantifier::ForAll);
    unclosed.populations[0].closure_identity = None;
    assert!(matches!(
        quire_verification::relational_hyperproperty::evaluate_relational(&unclosed),
        RelationalMethodOutcome::Incomplete(result)
            if result.code == RelationalNonConclusionCode::MissingPopulationClosure
    ));

    let mut missing = request(Quantifier::Exists, Quantifier::ForAll);
    missing.alignments.clear();
    assert!(matches!(
        quire_verification::relational_hyperproperty::evaluate_relational(&missing),
        RelationalMethodOutcome::Incomplete(result)
            if result.code == RelationalNonConclusionCode::MissingAlignment
    ));

    let mut ambiguous = request(Quantifier::Exists, Quantifier::ForAll);
    ambiguous.alignments.push(alignment("trace:a", "trace:b"));
    assert!(matches!(
        quire_verification::relational_hyperproperty::evaluate_relational(&ambiguous),
        RelationalMethodOutcome::Incomplete(result)
            if result.code == RelationalNonConclusionCode::AmbiguousAlignment
    ));

    let mut bounded = request(Quantifier::Exists, Quantifier::ForAll);
    bounded.finite_assignment_bound = 1;
    assert!(matches!(
        quire_verification::relational_hyperproperty::evaluate_relational(&bounded),
        RelationalMethodOutcome::RequiresBound(result)
            if result.code == RelationalNonConclusionCode::AssignmentBoundExceeded
    ));
}

/// Tracing: TC-048, FR-030-AC-3.
#[trace("TC-048", "FR-030-AC-3")]
#[test]
fn tc_048_never_correlates_by_local_name_or_reuses_one_trace_identity() {
    let explicit = request(Quantifier::Exists, Quantifier::ForAll);
    assert!(matches!(
        quire_verification::relational_hyperproperty::evaluate_relational(&explicit),
        RelationalMethodOutcome::Proved(_)
    ));

    let mut local_name_only = explicit.clone();
    local_name_only.alignments = vec![alignment("same-local-name", "same-local-name")];
    assert!(matches!(
        quire_verification::relational_hyperproperty::evaluate_relational(&local_name_only),
        RelationalMethodOutcome::Incomplete(result)
            if result.code == RelationalNonConclusionCode::MissingAlignment
    ));

    let mut duplicated = explicit.clone();
    duplicated.populations[0].traces[1].identity =
        duplicated.populations[0].traces[0].identity.clone();
    assert!(matches!(
        quire_verification::relational_hyperproperty::evaluate_relational(&duplicated),
        RelationalMethodOutcome::Refused(result)
            if result.code == RelationalNonConclusionCode::DuplicateTraceIdentity
    ));

    let mut one_trace = explicit;
    one_trace.populations[0].traces.pop();
    one_trace.alignments.clear();
    assert!(matches!(
        quire_verification::relational_hyperproperty::evaluate_relational(&one_trace),
        RelationalMethodOutcome::Incomplete(result)
            if result.code == RelationalNonConclusionCode::InsufficientDistinctTraces
    ));
}

/// Tracing: TC-048, FR-030-CON-2, FR-030-CON-3.
#[trace("TC-048", "FR-030-CON-2", "FR-030-CON-3")]
#[test]
fn tc_048_preserves_dependent_oracle_classification_without_promotion() {
    let request = request(Quantifier::Exists, Quantifier::ForAll);
    let outcome = quire_verification::relational_hyperproperty::evaluate_relational(&request);
    assert_eq!(
        outcome.oracle_dependence(),
        Some(&request.oracle_dependence)
    );
    assert!(matches!(
        outcome.oracle_dependence(),
        Some(OracleDependence::Shared { group_identity })
            if group_identity == "oracle-group:checked-runtime"
    ));
}
