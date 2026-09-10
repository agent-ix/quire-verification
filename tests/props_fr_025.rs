// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Property tests generated from the extractable FR-025 obligations.

mod support;

use ix_trace_rs::trace;
use proptest::prelude::*;
use quire_verification::qualification_technique::{
    QualificationTechniqueOutcome, qualify_technique_manifest,
};
use quire_verification::shared_reference::ValidatedSharedReference;
use serde_json::{Value, json};
use support::qualification_technique::{TechniqueFixture, fixture, native_gapped_fixture, proofs};

fn technique_proofs(manifest: &Value, catalog: &Value) -> Vec<ValidatedSharedReference> {
    let mut proof = proofs(manifest);
    proof.extend(proofs(catalog));
    proof
}

fn freeze(
    candidate: Value,
    fixture: &TechniqueFixture,
) -> Result<QualificationTechniqueOutcome, quire_verification::contracts::VerificationError> {
    let proof = technique_proofs(&candidate, &fixture.catalog);
    qualify_technique_manifest(
        candidate,
        &fixture.profile,
        &fixture.corpus,
        &fixture.oracle,
        &fixture.catalog,
        &proof,
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    /// Trace: FR-025-AC-1
    /// spec-correctness: row=FR-025-AC-1 property=universal extraction=extractable origin=regex
    #[trace("TC-042", "FR-025-AC-1")]
    #[test]
    fn prop_fr_025_ac_1_normalizes_sets_and_refuses_one_axis_binding_changes(
        reverse in any::<bool>(),
        changed_field in prop::sample::select(vec!["guaranteeId", "budgetId", "stoppingRule"]),
    ) {
        let fixture = fixture();
        let mut first = fixture.manifest.clone();
        first["configurations"][0]["limitations"] = json!([
            "Constraint alpha",
            "Constraint beta",
            "Constraint gamma"
        ]);
        let mut second = first.clone();
        if reverse {
            second["configurations"][0]["limitations"]
                .as_array_mut()
                .expect("limitations")
                .reverse();
        }
        let first = match freeze(first, &fixture).expect("first permutation") {
            QualificationTechniqueOutcome::Complete(manifest) => manifest,
            QualificationTechniqueOutcome::Incomplete { .. } => panic!("first permutation incomplete"),
        };
        let second = match freeze(second, &fixture).expect("second permutation") {
            QualificationTechniqueOutcome::Complete(manifest) => manifest,
            QualificationTechniqueOutcome::Incomplete { .. } => panic!("second permutation incomplete"),
        };
        prop_assert_eq!(first.identity(), second.identity());

        let mut changed = fixture.manifest.clone();
        changed["configurations"][0][changed_field] = json!(format!("changed-{changed_field}"));
        prop_assert!(freeze(changed, &fixture).is_err());

        let native = native_gapped_fixture();
        let native_is_incomplete = matches!(
            freeze(native.manifest.clone(), &native).expect("native admission result"),
            QualificationTechniqueOutcome::Incomplete { .. }
        );
        prop_assert!(native_is_incomplete);
    }

}
