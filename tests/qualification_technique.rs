// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! TC-042 qualification-technique manifest and case-identity tests.

mod support;

use ix_trace_rs::trace;
use quire_verification::contracts::{VerificationErrorCode, jcs_sha256};
use quire_verification::qualification_technique::{
    QualificationTechniqueOutcome, plan_qualification_case, qualify_technique_manifest,
};
use serde_json::{Value, json};
use support::qualification_technique::fixture_with;
use support::qualification_technique::{fixture, native_gapped_fixture, proofs, reference};

fn technique_proofs(
    manifest: &Value,
    catalog: &Value,
) -> Vec<quire_verification::shared_reference::ValidatedSharedReference> {
    let mut proof = proofs(manifest);
    proof.extend(proofs(catalog));
    proof
}

fn freeze(
    candidate: Value,
    fixture: &support::qualification_technique::TechniqueFixture,
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

#[trace("TC-042", "FR-025-AC-1")]
#[test]
fn tc_042_freezes_one_exact_configuration_per_obligation() {
    let fixture = fixture();
    let frozen = match freeze(fixture.manifest.clone(), &fixture).expect("technique decision") {
        QualificationTechniqueOutcome::Complete(manifest) => manifest,
        QualificationTechniqueOutcome::Incomplete { gaps, .. } => {
            panic!("complete technique manifest returned gaps: {gaps:?}")
        }
    };
    assert!(frozen.identity().starts_with("sha256-jcs:"));
    assert_eq!(
        serde_json::to_value(frozen.manifest()).expect("manifest JSON")["manifestId"],
        "QTM-001"
    );

    let mut permuted = fixture.manifest.clone();
    permuted["configurations"][0]["limitations"] = json!([
        "A second set-like limitation",
        "The property covers the retained syntax population"
    ]);
    let mut reversed = permuted.clone();
    reversed["configurations"][0]["limitations"]
        .as_array_mut()
        .expect("limitations")
        .reverse();
    let left = match freeze(permuted, &fixture).expect("left manifest") {
        QualificationTechniqueOutcome::Complete(manifest) => manifest,
        QualificationTechniqueOutcome::Incomplete { .. } => panic!("left manifest incomplete"),
    };
    let right = match freeze(reversed, &fixture).expect("right manifest") {
        QualificationTechniqueOutcome::Complete(manifest) => manifest,
        QualificationTechniqueOutcome::Incomplete { .. } => panic!("right manifest incomplete"),
    };
    assert_eq!(left.identity(), right.identity());
}

#[trace("TC-042", "FR-025-AC-1")]
#[test]
fn tc_042_reports_carried_native_admission_gaps_without_assuming_coverage() {
    let gapped_fixture = native_gapped_fixture();
    let gaps =
        match freeze(gapped_fixture.manifest.clone(), &gapped_fixture).expect("gapped decision") {
            QualificationTechniqueOutcome::Complete(_) => panic!("gapped manifest completed"),
            QualificationTechniqueOutcome::Incomplete { gaps, .. } => gaps,
        };
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].obligation_id(), "syntax-property");
    assert_eq!(
        gaps[0].admission_rules(),
        [Box::<str>::from("numeric-domain")]
    );

    let mut missing = gapped_fixture.manifest.clone();
    missing["gaps"][0]["admissionRules"] = json!([]);
    let error = freeze(missing, &gapped_fixture).expect_err("missing admission gap accepted");
    assert_eq!(
        error.code(),
        VerificationErrorCode::InvalidQualificationTechniqueManifest
    );

    let mut configuration = gapped_fixture.manifest.clone();
    configuration["configurations"] = fixture().manifest["configurations"].clone();
    configuration["gaps"] = json!([]);
    let error =
        freeze(configuration, &gapped_fixture).expect_err("configuration claimed open admission");
    assert_eq!(
        error.code(),
        VerificationErrorCode::InvalidQualificationTechniqueManifest
    );
}

#[trace("TC-042", "FR-025-AC-1")]
#[test]
fn tc_042_schema_discriminates_required_and_family_specific_fields() {
    let base_fixture = fixture();
    for field in [
        "contractVersion",
        "schemaVersion",
        "kind",
        "manifestId",
        "profileIdentity",
        "corpusIdentity",
        "oracleManifestIdentity",
        "catalogIdentity",
        "configurations",
        "gaps",
    ] {
        let mut candidate = base_fixture.manifest.clone();
        candidate
            .as_object_mut()
            .expect("manifest object")
            .remove(field);
        assert!(
            freeze(candidate, &base_fixture).is_err(),
            "missing {field} accepted"
        );
    }

    for field in [
        "configurationId",
        "obligationId",
        "guaranteeId",
        "techniqueId",
        "propertyRef",
        "inputProjection",
        "budgetId",
        "stoppingRule",
        "harnessRef",
        "requestTemplateRef",
        "limitations",
        "technique",
    ] {
        let mut candidate = base_fixture.manifest.clone();
        candidate["configurations"][0]
            .as_object_mut()
            .expect("configuration object")
            .remove(field);
        assert!(
            freeze(candidate, &base_fixture).is_err(),
            "missing {field} accepted"
        );
    }

    for field in [
        "family",
        "generatorRef",
        "shrinkerRef",
        "seed",
        "maximumCases",
        "maximumShrinks",
    ] {
        let mut candidate = base_fixture.manifest.clone();
        candidate["configurations"][0]["technique"]
            .as_object_mut()
            .expect("technique object")
            .remove(field);
        assert!(
            freeze(candidate, &base_fixture).is_err(),
            "missing {field} accepted"
        );
    }

    let mut cross_tagged = base_fixture.manifest.clone();
    cross_tagged["configurations"][0]["technique"]["targetRef"] =
        reference("executable-projection", "target:foreign-family");
    assert!(freeze(cross_tagged, &base_fixture).is_err());

    for bad_seed in ["00", "18446744073709551616"] {
        let mut candidate = base_fixture.manifest.clone();
        candidate["configurations"][0]["technique"]["seed"] = json!(bad_seed);
        assert!(
            freeze(candidate, &base_fixture).is_err(),
            "seed {bad_seed} accepted"
        );
    }
    for bad_limit in [0_u64, 100_001] {
        let mut candidate = base_fixture.manifest.clone();
        candidate["configurations"][0]["technique"]["maximumCases"] = json!(bad_limit);
        assert!(
            freeze(candidate, &base_fixture).is_err(),
            "limit {bad_limit} accepted"
        );
    }
}

#[trace("TC-042", "FR-025-AC-1")]
#[test]
fn tc_042_refuses_conflicting_coverage_references_and_catalog_methods() {
    let base_fixture = fixture();

    let foreign_identity = format!("sha256-jcs:{}", "0".repeat(64));
    for field in [
        "profileIdentity",
        "corpusIdentity",
        "oracleManifestIdentity",
        "catalogIdentity",
    ] {
        let mut changed = base_fixture.manifest.clone();
        changed[field] = json!(&foreign_identity);
        assert!(
            freeze(changed, &base_fixture).is_err(),
            "changed {field} accepted"
        );
    }

    let error = qualify_technique_manifest(
        base_fixture.manifest.clone(),
        &base_fixture.profile,
        &base_fixture.corpus,
        &base_fixture.oracle,
        &base_fixture.catalog,
        &[],
    )
    .expect_err("missing strict-reader proofs accepted");
    assert_eq!(
        error.code(),
        VerificationErrorCode::QualificationTechniqueUnavailable
    );

    let mut both = base_fixture.manifest.clone();
    both["gaps"] = json!([{
        "gapId": "duplicate-coverage",
        "obligationId": "syntax-property",
        "owner": "owner:technique",
        "reason": "Duplicate coverage must refuse",
        "limitations": ["The configuration already covers this obligation"],
        "admissionRules": []
    }]);
    assert!(freeze(both, &base_fixture).is_err());

    let mut wrong_projection = base_fixture.manifest.clone();
    wrong_projection["configurations"][0]["inputProjection"]["entryIds"] = json!(["absent"]);
    assert!(freeze(wrong_projection, &base_fixture).is_err());

    let mut wrong_role = base_fixture.manifest.clone();
    wrong_role["configurations"][0]["propertyRef"]["value"]["kind"] = json!("source");
    assert!(freeze(wrong_role, &base_fixture).is_err());

    let mut no_method_fixture = fixture();
    no_method_fixture.catalog["catalog"]["methods"]
        .as_array_mut()
        .expect("methods")
        .retain(|method| method["id"] != "property-based-testing");
    no_method_fixture.manifest["catalogIdentity"] =
        json!(jcs_sha256(&no_method_fixture.catalog).expect("catalog identity"));
    let error = freeze(no_method_fixture.manifest.clone(), &no_method_fixture)
        .expect_err("missing catalog method accepted");
    assert_eq!(
        error.code(),
        VerificationErrorCode::QualificationTechniqueUnavailable
    );
}

#[trace("TC-042")]
#[test]
fn tc_042_case_identity_preserves_ordered_shrinks_and_normalizes_prior_sets() {
    let fixture = fixture();
    let frozen = match freeze(fixture.manifest.clone(), &fixture).expect("technique decision") {
        QualificationTechniqueOutcome::Complete(manifest) => manifest,
        QualificationTechniqueOutcome::Incomplete { .. } => panic!("manifest incomplete"),
    };
    let observation_a = reference("observation", "observation:a");
    let observation_b = reference("observation", "observation:b");
    let shrink_a = reference("generated-artifact", "shrink:a");
    let shrink_b = reference("generated-artifact", "shrink:b");
    let refs = json!([observation_a, observation_b, shrink_a, shrink_b]);
    let proof = proofs(&refs);

    let identity = plan_qualification_case(
        &frozen,
        "property-syntax",
        "0",
        &[refs[0].clone(), refs[1].clone()],
        &[refs[2].clone(), refs[3].clone()],
        &proof,
    )
    .expect("case identity");
    let prior_permuted = plan_qualification_case(
        &frozen,
        "property-syntax",
        "0",
        &[refs[1].clone(), refs[0].clone()],
        &[refs[2].clone(), refs[3].clone()],
        &proof,
    )
    .expect("permuted prior identity");
    assert_eq!(identity.identity(), prior_permuted.identity());
    let request = serde_json::to_value(identity.request()).expect("case request JSON");
    assert_eq!(request["kind"], "QualificationCaseRequest");
    assert_eq!(request["caseOrdinal"], "0");
    assert_eq!(request["configurationId"], "property-syntax");
    assert!(
        request["configurationIdentity"]
            .as_str()
            .is_some_and(|value| value.starts_with("sha256-jcs:"))
    );

    let shrink_permuted = plan_qualification_case(
        &frozen,
        "property-syntax",
        "0",
        &[refs[0].clone(), refs[1].clone()],
        &[refs[3].clone(), refs[2].clone()],
        &proof,
    )
    .expect("permuted shrink identity");
    assert_ne!(identity.identity(), shrink_permuted.identity());

    let next = plan_qualification_case(&frozen, "property-syntax", "31", &[], &[], &[])
        .expect("last admitted ordinal");
    assert_ne!(identity.identity(), next.identity());
    for bad in ["32", "00", "18446744073709551616"] {
        assert!(
            plan_qualification_case(&frozen, "property-syntax", bad, &[], &[], &[]).is_err(),
            "ordinal {bad} accepted"
        );
    }
}

#[trace("TC-042")]
#[test]
fn tc_042_plans_example_and_metamorphic_and_blocks_unpublished_differential() {
    let example = fixture_with(
        "bdd-spec-by-example",
        "execute each projected example exactly once",
        &json!({"family": "example"}),
    );
    let example_manifest = match freeze(example.manifest.clone(), &example).expect("example") {
        QualificationTechniqueOutcome::Complete(manifest) => manifest,
        QualificationTechniqueOutcome::Incomplete { .. } => panic!("example incomplete"),
    };
    assert!(
        plan_qualification_case(&example_manifest, "property-syntax", "0", &[], &[], &[]).is_ok()
    );
    assert!(
        plan_qualification_case(&example_manifest, "property-syntax", "1", &[], &[], &[]).is_err()
    );

    let metamorphic = fixture_with(
        "metamorphic-testing",
        "execute exactly 8 transformed cases or stop at the first violated relation",
        &json!({
            "family": "metamorphic",
            "relationProcedureRef": reference("executable-projection", "relation:syntax"),
            "transformRef": reference("executable-projection", "transform:syntax"),
            "seed": "7",
            "maximumCases": 8,
            "maximumShrinks": 4
        }),
    );
    let metamorphic_manifest =
        match freeze(metamorphic.manifest.clone(), &metamorphic).expect("metamorphic") {
            QualificationTechniqueOutcome::Complete(manifest) => manifest,
            QualificationTechniqueOutcome::Incomplete { .. } => panic!("metamorphic incomplete"),
        };
    assert!(
        plan_qualification_case(&metamorphic_manifest, "property-syntax", "7", &[], &[], &[])
            .is_ok()
    );

    let differential = fixture_with(
        "differential-testing",
        "execute exactly 8 comparisons or stop at the first disagreement",
        &json!({
            "family": "differential",
            "oracleBindingIds": ["unavailable-oracle"],
            "comparisonProcedureRef": reference("executable-projection", "comparison:syntax"),
            "seed": "9",
            "maximumCases": 8,
            "maximumShrinks": 4
        }),
    );
    let error = freeze(differential.manifest.clone(), &differential)
        .expect_err("unpublished differential method accepted");
    assert_eq!(
        error.code(),
        VerificationErrorCode::QualificationTechniqueUnavailable
    );
}
