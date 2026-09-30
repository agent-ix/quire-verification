// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Typed Engineering Assurance execution for qualification-oracle observations.
//!
//! Governing requirement: FR-024.

use std::collections::BTreeSet;
use std::io::Read;

use engineering_assurance::producer_execution::{
    ArgumentBinding, CancellationToken, ContentDigest, ContractBinding, ExitCodeBinding,
    InvalidExecutionRequest, MalformedResponse, OutputArtifact, PRODUCER_EXECUTION_RESULT_PROTOCOL,
    ProcessEvidence, ProducerExecutionRequest, ProducerExecutionResult, ProducerExecutionState,
    ProducerExecutor, ProducerResponseAdapter, ResponseBinding,
};
use serde::Serialize;
use serde_json::{Value, json};

use crate::contracts::{
    MAX_JSON_BYTES, VerificationError, VerificationErrorCode, jcs_canonicalize, jcs_sha256,
    validate_schema_definition, wire,
};
use crate::qualification_corpus::FrozenQualificationCorpus;
use crate::qualification_oracle::FrozenQualificationOracleManifest;
use crate::qualification_profile::FrozenQualificationProfile;
use crate::shared_reference::{ValidatedSharedReference, parse_strict_bounded_json};

/// Exact Engineering Assurance source revision compiled into this consumer.
pub const ENGINEERING_ASSURANCE_REVISION: &str = env!("ENGINEERING_ASSURANCE_REVISION");

const RESPONSE_ROLE: &str = "response";
const ORACLE_RESPONSE_PROTOCOL: &str = "quire-verification.qualification-oracle-response/v1";
const COMPARISON_RESPONSE_PROTOCOL: &str =
    "quire-verification.qualification-comparison-response/v1";
const ADAPTER_VERSION: &str = "1";
const CONTRACT_BYTES: &[u8] = include_bytes!("../contracts/e02-draft-2.schema.json");
const ADAPTER_BYTES: &[u8] = include_bytes!("qualification_oracle_execution.rs");

/// Caller-owned adapter for an exact oracle observation response.
#[derive(Clone, Debug)]
pub struct QualificationOracleResponseAdapter {
    binding: ResponseBinding,
}

impl QualificationOracleResponseAdapter {
    /// Constructs the exact response binding implemented by this crate revision.
    #[must_use]
    pub fn new() -> Self {
        Self {
            binding: response_binding(
                ORACLE_RESPONSE_PROTOCOL,
                "qualification-oracle-response-adapter",
            ),
        }
    }
}

impl Default for QualificationOracleResponseAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProducerResponseAdapter for QualificationOracleResponseAdapter {
    type Observation = wire::QualificationOracleProducerResponse;

    fn binding(&self) -> &ResponseBinding {
        &self.binding
    }

    fn decode(
        &self,
        process: &ProcessEvidence,
        artifacts: &[OutputArtifact],
    ) -> Result<Self::Observation, MalformedResponse> {
        decode_response("QualificationOracleProducerResponse", process, artifacts)
    }
}

/// Caller-owned adapter for an exact oracle comparison response.
#[derive(Clone, Debug)]
pub struct QualificationComparisonResponseAdapter {
    binding: ResponseBinding,
}

impl QualificationComparisonResponseAdapter {
    /// Constructs the exact response binding implemented by this crate revision.
    #[must_use]
    pub fn new() -> Self {
        Self {
            binding: response_binding(
                COMPARISON_RESPONSE_PROTOCOL,
                "qualification-comparison-response-adapter",
            ),
        }
    }
}

impl Default for QualificationComparisonResponseAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProducerResponseAdapter for QualificationComparisonResponseAdapter {
    type Observation = wire::QualificationComparisonProducerResponse;

    fn binding(&self) -> &ResponseBinding {
        &self.binding
    }

    fn decode(
        &self,
        process: &ProcessEvidence,
        artifacts: &[OutputArtifact],
    ) -> Result<Self::Observation, MalformedResponse> {
        decode_response(
            "QualificationComparisonProducerResponse",
            process,
            artifacts,
        )
    }
}

/// Returns the exact oracle-response binding callers must place in the request.
#[must_use]
pub fn qualification_oracle_response_binding() -> ResponseBinding {
    QualificationOracleResponseAdapter::new().binding
}

/// Returns the exact comparison-response binding callers must place in the request.
#[must_use]
pub fn qualification_comparison_response_binding() -> ResponseBinding {
    QualificationComparisonResponseAdapter::new().binding
}

/// Submits one oracle request through the accepted Engineering Assurance executor.
///
/// # Errors
///
/// Returns the Engineering Assurance structural refusal when the request cannot mint
/// an identity-bearing result.
pub fn execute_qualification_oracle(
    executor: &ProducerExecutor,
    request: &ProducerExecutionRequest,
    cancellation: &CancellationToken,
) -> Result<
    ProducerExecutionResult<wire::QualificationOracleProducerResponse>,
    InvalidExecutionRequest,
> {
    executor.execute(
        request,
        cancellation,
        &QualificationOracleResponseAdapter::new(),
    )
}

/// Submits one comparison request through the accepted Engineering Assurance executor.
///
/// # Errors
///
/// Returns the Engineering Assurance structural refusal when the request cannot mint
/// an identity-bearing result.
pub fn execute_qualification_comparison(
    executor: &ProducerExecutor,
    request: &ProducerExecutionRequest,
    cancellation: &CancellationToken,
) -> Result<
    ProducerExecutionResult<wire::QualificationComparisonProducerResponse>,
    InvalidExecutionRequest,
> {
    executor.execute(
        request,
        cancellation,
        &QualificationComparisonResponseAdapter::new(),
    )
}

/// Completed comparison or the exact non-completed execution finding.
#[derive(Debug)]
pub enum QualificationComparisonAssessment {
    /// The comparison producer completed and returned a bound comparison.
    Compared(Box<wire::QualificationOracleComparisonRecord>),
    /// Engineering Assurance returned a non-completed state.
    ExecutionFinding(Box<wire::QualificationOracleExecutionRecord>),
}

/// Frozen qualification context and strict references for one Engineering Assurance result.
#[derive(Clone, Copy, Debug)]
pub struct QualificationExecutionEvidence<'a> {
    /// Frozen oracle manifest that selected the request.
    pub manifest: &'a FrozenQualificationOracleManifest,
    /// Stable binding identifier within the frozen manifest.
    pub binding_id: &'a str,
    /// Frozen production-qualification profile.
    pub profile: &'a FrozenQualificationProfile,
    /// Frozen retained qualification corpus.
    pub corpus: &'a FrozenQualificationCorpus,
    /// Exact request submitted through Engineering Assurance.
    pub request: &'a ProducerExecutionRequest,
    /// Strict run-artifact reference retaining the canonical Engineering Assurance result.
    pub result_ref: &'a Value,
    /// Strict-reader proofs needed by the selected profile, manifest, request, and result.
    pub validated_references: &'a [ValidatedSharedReference],
}

/// Validates and projects one typed Engineering Assurance oracle result.
///
/// # Errors
///
/// Returns `invalid_qualification_oracle_binding` unless every compiled interface,
/// request, result, executable, corpus, and observation identity is exact.
pub fn record_qualification_oracle_execution(
    evidence: &QualificationExecutionEvidence<'_>,
    result: &ProducerExecutionResult<wire::QualificationOracleProducerResponse>,
) -> Result<wire::QualificationOracleExecutionRecord, VerificationError> {
    let context = validate_execution_context(Operation::Oracle, evidence, result)?;
    let state = match &result.state {
        ProducerExecutionState::Completed { observation } => {
            validate_completed_observation(
                result,
                observation,
                &QualificationOracleResponseAdapter::new(),
            )?;
            let response = serde_json::to_value(observation).map_err(encoding_refusal)?;
            validate_response(&context, &response, evidence.validated_references)?;
            json!({"kind": "completed", "observationRef": response["observationRef"]})
        }
        state => noncompleted_state(state)?,
    };
    execution_record(&context, &state)
}

/// Validates and projects one typed Engineering Assurance comparison result.
///
/// # Errors
///
/// Returns `invalid_qualification_oracle_binding` unless every compiled interface,
/// request, result, executable, corpus, relation, and observation identity is exact.
pub fn record_qualification_comparison_execution(
    evidence: &QualificationExecutionEvidence<'_>,
    result: &ProducerExecutionResult<wire::QualificationComparisonProducerResponse>,
) -> Result<QualificationComparisonAssessment, VerificationError> {
    let context = validate_execution_context(Operation::Comparison, evidence, result)?;
    let ProducerExecutionState::Completed { observation } = &result.state else {
        let state = noncompleted_state(&result.state)?;
        return execution_record(&context, &state)
            .map(|record| QualificationComparisonAssessment::ExecutionFinding(Box::new(record)));
    };
    validate_completed_observation(
        result,
        observation,
        &QualificationComparisonResponseAdapter::new(),
    )?;
    let response = serde_json::to_value(observation).map_err(encoding_refusal)?;
    validate_response(&context, &response, evidence.validated_references)?;
    let outcome = response["outcome"].clone();
    let value = json!({
        "contractVersion": wire::CONTRACT_VERSION,
        "schemaVersion": "quire-verification.qualification-oracle-comparison/v1",
        "kind": "QualificationOracleComparisonRecord",
        "manifestIdentity": context.manifest_identity,
        "bindingId": context.binding["bindingId"],
        "guaranteeId": context.binding["guaranteeId"],
        "inputProjection": context.projection,
        "engineeringAssuranceRevision": ENGINEERING_ASSURANCE_REVISION,
        "executionProtocol": PRODUCER_EXECUTION_RESULT_PROTOCOL,
        "requestIdentity": context.request_identity,
        "resultIdentity": context.result_identity,
        "resultRef": context.result_ref,
        "expectedRelation": context.binding["expectedRelation"],
        "candidateObservationRef": response["candidateObservationRef"],
        "oracleObservationRef": response["oracleObservationRef"],
        "outcome": outcome,
        "resolution": if response["outcome"] == "agreed" { "resolved" } else { "unresolved" }
    });
    validate_schema_definition("QualificationOracleComparisonRecord", &value).map_err(|error| {
        invalid_binding("comparison record violates its schema", &error.to_string())
    })?;
    serde_json::from_value(value)
        .map(Box::new)
        .map(QualificationComparisonAssessment::Compared)
        .map_err(encoding_refusal)
}

/// Binds retained disagreement analysis to the exact procedure declared by the oracle manifest.
///
/// # Errors
///
/// Returns `invalid_qualification_oracle_binding` unless the comparison is an unresolved
/// disagreement and the canonical procedure result, declared procedure, and strict references
/// all match exactly. The closed response schema has no qualification-disposition field.
pub fn record_qualification_disagreement(
    manifest: &FrozenQualificationOracleManifest,
    binding_id: &str,
    comparison: &wire::QualificationOracleComparisonRecord,
    result_ref: &Value,
    validated_references: &[ValidatedSharedReference],
) -> Result<wire::QualificationOracleDisagreementRecord, VerificationError> {
    let manifest_value = serde_json::to_value(manifest.manifest()).map_err(encoding_refusal)?;
    let binding = manifest_value["bindings"]
        .as_array()
        .and_then(|bindings| {
            bindings
                .iter()
                .find(|binding| binding["bindingId"] == binding_id)
        })
        .ok_or_else(|| invalid_binding("oracle binding is absent", binding_id))?;
    let comparison_value = serde_json::to_value(comparison).map_err(encoding_refusal)?;
    validate_schema_definition("QualificationOracleComparisonRecord", &comparison_value).map_err(
        |error| invalid_binding("comparison record violates its schema", &error.to_string()),
    )?;
    if comparison_value["manifestIdentity"] != manifest.identity()
        || comparison_value["bindingId"] != binding["bindingId"]
        || comparison_value["guaranteeId"] != binding["guaranteeId"]
        || comparison_value["outcome"] != "disagreed"
        || comparison_value["resolution"] != "unresolved"
    {
        return Err(invalid_binding(
            "disagreement procedure requires the exact unresolved comparison",
            binding_id,
        ));
    }
    let comparison_identity = jcs_sha256(&comparison_value)?;
    let result_bytes = require_reference_bytes(result_ref, "run-artifact", validated_references)?;
    let response = parse_strict_bounded_json(result_bytes).map_err(|error| {
        invalid_binding(
            "disagreement result is not strict bounded JSON",
            &error.to_string(),
        )
    })?;
    validate_schema_definition("QualificationDisagreementProcedureResponse", &response).map_err(
        |error| {
            invalid_binding(
                "disagreement result violates its schema",
                &error.to_string(),
            )
        },
    )?;
    if jcs_canonicalize(&response)? != result_bytes {
        return Err(invalid_binding(
            "disagreement result is not retained in canonical form",
            binding_id,
        ));
    }
    if response["bindingId"] != binding["bindingId"]
        || response["comparisonIdentity"] != comparison_identity.as_ref()
        || response["procedureRef"] != binding["disagreementProcedureRef"]
    {
        return Err(invalid_binding(
            "disagreement result differs from the selected comparison or procedure",
            binding_id,
        ));
    }
    require_reference_bytes(
        &response["procedureRef"],
        "review-procedure",
        validated_references,
    )?;
    let sorted_strings = |field: &str| -> Result<Value, VerificationError> {
        let values = response[field]
            .as_array()
            .ok_or_else(|| invalid_binding("disagreement result member is not an array", field))?;
        let sorted = values
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| invalid_binding("disagreement result member is not text", field))
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        Ok(json!(sorted))
    };
    let value = json!({
        "contractVersion": wire::CONTRACT_VERSION,
        "schemaVersion": "quire-verification.qualification-disagreement/v1",
        "kind": "QualificationOracleDisagreementRecord",
        "manifestIdentity": manifest.identity(),
        "bindingId": binding["bindingId"],
        "guaranteeId": binding["guaranteeId"],
        "comparisonIdentity": comparison_identity,
        "procedureRef": response["procedureRef"],
        "resultRef": result_ref,
        "attributions": sorted_strings("attributions")?,
        "bounds": sorted_strings("bounds")?
    });
    validate_schema_definition("QualificationOracleDisagreementRecord", &value).map_err(
        |error| {
            invalid_binding(
                "disagreement record violates its schema",
                &error.to_string(),
            )
        },
    )?;
    serde_json::from_value(value).map_err(encoding_refusal)
}

#[derive(Clone, Copy)]
enum Operation {
    Oracle,
    Comparison,
}

impl Operation {
    const fn name(self) -> &'static str {
        match self {
            Self::Oracle => "oracle",
            Self::Comparison => "comparison",
        }
    }

    const fn request_field(self) -> &'static str {
        match self {
            Self::Oracle => "executionRequestRef",
            Self::Comparison => "comparisonRequestRef",
        }
    }

    const fn executable_field(self) -> &'static str {
        match self {
            Self::Oracle => "executableRef",
            Self::Comparison => "comparisonExecutableRef",
        }
    }

    fn response_binding(self) -> ResponseBinding {
        match self {
            Self::Oracle => qualification_oracle_response_binding(),
            Self::Comparison => qualification_comparison_response_binding(),
        }
    }
}

struct ExecutionContext {
    operation: Operation,
    manifest_identity: String,
    binding: Value,
    projection: Value,
    request_identity: Value,
    result_identity: Value,
    result_ref: Value,
}

fn validate_execution_context<T: Serialize>(
    operation: Operation,
    evidence: &QualificationExecutionEvidence<'_>,
    result: &ProducerExecutionResult<T>,
) -> Result<ExecutionContext, VerificationError> {
    validate_engineering_assurance_interface(evidence.profile, evidence.validated_references)?;
    let manifest_value =
        serde_json::to_value(evidence.manifest.manifest()).map_err(encoding_refusal)?;
    let binding = manifest_value["bindings"]
        .as_array()
        .and_then(|bindings| {
            bindings
                .iter()
                .find(|binding| binding["bindingId"] == evidence.binding_id)
        })
        .cloned()
        .ok_or_else(|| invalid_binding("oracle binding is absent", evidence.binding_id))?;
    if evidence.request.response != operation.response_binding() {
        return Err(invalid_binding(
            "execution request selects a foreign response binding",
            operation.name(),
        ));
    }
    validate_response_output(evidence.request)?;
    validate_executable(
        &binding[operation.executable_field()],
        evidence.request,
        evidence.validated_references,
    )?;
    let request_bytes = jcs_canonicalize(evidence.request)?;
    if require_reference_bytes(
        &binding[operation.request_field()],
        "invocation",
        evidence.validated_references,
    )? != request_bytes
    {
        return Err(invalid_binding(
            "execution request reference does not retain the canonical request",
            operation.request_field(),
        ));
    }
    let request_identity = evidence.request.identity().map_err(|error| {
        invalid_binding(
            "execution request cannot mint an identity",
            &error.to_string(),
        )
    })?;
    if result.protocol != PRODUCER_EXECUTION_RESULT_PROTOCOL
        || result.request_identity != request_identity
        || result.producer != evidence.request.producer
    {
        return Err(invalid_binding(
            "Engineering Assurance result does not match the exact request",
            operation.name(),
        ));
    }
    let result_bytes = result.canonical_bytes().map_err(|error| {
        invalid_binding("execution result is not canonical", &error.to_string())
    })?;
    if require_reference_bytes(
        evidence.result_ref,
        "run-artifact",
        evidence.validated_references,
    )? != result_bytes
    {
        return Err(invalid_binding(
            "execution result reference does not retain the canonical result",
            operation.name(),
        ));
    }
    let result_identity = result
        .identity()
        .map_err(|error| invalid_binding("execution result has no identity", &error.to_string()))?;
    let projection = validate_projection(
        &binding,
        evidence.corpus,
        evidence.request,
        evidence.validated_references,
    )?;
    Ok(ExecutionContext {
        operation,
        manifest_identity: evidence.manifest.identity().to_owned(),
        binding,
        projection,
        request_identity: serde_json::to_value(request_identity).map_err(encoding_refusal)?,
        result_identity: serde_json::to_value(result_identity).map_err(encoding_refusal)?,
        result_ref: evidence.result_ref.clone(),
    })
}

fn validate_completed_observation<A>(
    result: &ProducerExecutionResult<A::Observation>,
    observation: &A::Observation,
    adapter: &A,
) -> Result<(), VerificationError>
where
    A: ProducerResponseAdapter,
    A::Observation: Serialize,
{
    if result.observed_executable_digest.as_ref() != Some(&result.producer.executable_digest) {
        return Err(invalid_binding(
            "completed result lacks the exact observed executable identity",
            &result.producer.name,
        ));
    }
    let process = result.process.as_ref().ok_or_else(|| {
        invalid_binding(
            "completed result lacks retained process evidence",
            &result.producer.name,
        )
    })?;
    let retained = adapter.decode(process, &result.artifacts).map_err(|_| {
        invalid_binding(
            "completed observation differs from retained producer output",
            &result.producer.name,
        )
    })?;
    if jcs_canonicalize(&retained)? != jcs_canonicalize(observation)? {
        return Err(invalid_binding(
            "completed observation differs from retained producer output",
            &result.producer.name,
        ));
    }
    Ok(())
}

fn validate_engineering_assurance_interface(
    profile: &FrozenQualificationProfile,
    proofs: &[ValidatedSharedReference],
) -> Result<(), VerificationError> {
    let profile = serde_json::to_value(profile.profile()).map_err(encoding_refusal)?;
    let interface = profile["interfaces"]
        .as_array()
        .and_then(|interfaces| {
            interfaces
                .iter()
                .find(|interface| interface["role"] == "engineering-assurance")
        })
        .ok_or_else(|| {
            invalid_binding("Engineering Assurance interface is absent", "interfaces")
        })?;
    let reference = &interface["interfaceRef"];
    require_reference_bytes(reference, "source", proofs)?;
    Ok(())
}

fn validate_response_output(request: &ProducerExecutionRequest) -> Result<(), VerificationError> {
    let output_is_exact = request.outputs.len() == 1
        && request.outputs[0].role == RESPONSE_ROLE
        && request.outputs[0].required
        && request.arguments.iter().any(
            |argument| matches!(argument, ArgumentBinding::OutputArtifact { role } if role == RESPONSE_ROLE),
        );
    if !output_is_exact {
        return Err(invalid_binding(
            "execution request does not bind the required retained response output",
            RESPONSE_ROLE,
        ));
    }
    Ok(())
}

fn validate_executable(
    reference: &Value,
    request: &ProducerExecutionRequest,
    proofs: &[ValidatedSharedReference],
) -> Result<(), VerificationError> {
    let bytes = require_reference_bytes(reference, "executable-projection", proofs)?;
    if ContentDigest::of_bytes(bytes) != request.producer.executable_digest
        || reference["value"]["revision"]["value"] != request.producer.source_revision
    {
        return Err(invalid_binding(
            "execution request selects a different retained executable",
            &request.producer.name,
        ));
    }
    Ok(())
}

fn validate_projection(
    binding: &Value,
    corpus: &FrozenQualificationCorpus,
    request: &ProducerExecutionRequest,
    proofs: &[ValidatedSharedReference],
) -> Result<Value, VerificationError> {
    let declared = binding["inputProjection"]["entryIds"]
        .as_array()
        .ok_or_else(|| invalid_binding("binding projection is not an array", "entryIds"))?;
    let mode = binding["inputProjection"]["mode"]
        .as_str()
        .ok_or_else(|| invalid_binding("binding projection mode is absent", "mode"))?;
    let executed = request
        .inputs
        .iter()
        .map(|input| {
            input
                .role
                .strip_prefix("entry:")
                .ok_or_else(|| invalid_binding("request input has a foreign role", &input.role))
                .map(|entry_id| (entry_id, input))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let executed_ids = executed
        .iter()
        .map(|(entry_id, _)| Value::String((*entry_id).to_owned()))
        .collect::<Vec<_>>();
    let valid_shape = match mode {
        "ordered-batch" => executed_ids == declared.as_slice(),
        "each-entry" => {
            executed_ids.len() == 1 && declared.iter().any(|entry| entry == &executed_ids[0])
        }
        _ => false,
    };
    if !valid_shape {
        return Err(invalid_binding(
            "execution inputs differ from the declared corpus projection",
            mode,
        ));
    }
    let corpus = serde_json::to_value(corpus.manifest()).map_err(encoding_refusal)?;
    for (entry_id, input) in executed {
        let entry = corpus["entries"]
            .as_array()
            .and_then(|entries| entries.iter().find(|entry| entry["entryId"] == entry_id))
            .ok_or_else(|| invalid_binding("projected corpus entry is absent", entry_id))?;
        let bytes = require_reference_bytes(&entry["artifactRef"], "source", proofs)?;
        if ContentDigest::of_bytes(bytes) != input.digest {
            return Err(invalid_binding(
                "request input differs from retained corpus bytes",
                entry_id,
            ));
        }
    }
    Ok(json!({"mode": mode, "entryIds": executed_ids}))
}

fn validate_response(
    context: &ExecutionContext,
    response: &Value,
    proofs: &[ValidatedSharedReference],
) -> Result<(), VerificationError> {
    let fields_match = response["bindingId"] == context.binding["bindingId"]
        && response["guaranteeId"] == context.binding["guaranteeId"]
        && response["inputProjection"] == context.projection
        && response["relationId"] == context.binding["expectedRelation"]["relationId"];
    if !fields_match {
        return Err(invalid_binding(
            "producer response differs from the selected oracle binding",
            context.operation.name(),
        ));
    }
    let reference_fields: &[&str] = match context.operation {
        Operation::Oracle => &["observationRef"],
        Operation::Comparison => &["candidateObservationRef", "oracleObservationRef"],
    };
    for field in reference_fields {
        require_reference_bytes(&response[*field], "observation", proofs)?;
    }
    Ok(())
}

fn execution_record(
    context: &ExecutionContext,
    state: &Value,
) -> Result<wire::QualificationOracleExecutionRecord, VerificationError> {
    let value = json!({
        "contractVersion": wire::CONTRACT_VERSION,
        "schemaVersion": "quire-verification.qualification-oracle-execution/v1",
        "kind": "QualificationOracleExecutionRecord",
        "operation": context.operation.name(),
        "manifestIdentity": context.manifest_identity,
        "bindingId": context.binding["bindingId"],
        "guaranteeId": context.binding["guaranteeId"],
        "inputProjection": context.projection,
        "engineeringAssuranceRevision": ENGINEERING_ASSURANCE_REVISION,
        "executionProtocol": PRODUCER_EXECUTION_RESULT_PROTOCOL,
        "requestIdentity": context.request_identity,
        "resultIdentity": context.result_identity,
        "resultRef": context.result_ref,
        "state": state
    });
    validate_schema_definition("QualificationOracleExecutionRecord", &value).map_err(|error| {
        invalid_binding("execution record violates its schema", &error.to_string())
    })?;
    serde_json::from_value(value).map_err(encoding_refusal)
}

fn noncompleted_state<T: Serialize>(
    state: &ProducerExecutionState<T>,
) -> Result<Value, VerificationError> {
    if matches!(state, ProducerExecutionState::Completed { .. }) {
        return Err(invalid_binding(
            "completed result requires a typed observation",
            "completed",
        ));
    }
    serde_json::to_value(state).map_err(encoding_refusal)
}

fn decode_response<T>(
    definition: &str,
    process: &ProcessEvidence,
    artifacts: &[OutputArtifact],
) -> Result<T, MalformedResponse>
where
    T: serde::de::DeserializeOwned,
{
    if !process.stdout.bytes.is_empty()
        || artifacts.len() != 1
        || artifacts[0].role != RESPONSE_ROLE
    {
        return Err(MalformedResponse);
    }
    let artifact = &artifacts[0];
    if artifact.byte_length > u64::try_from(MAX_JSON_BYTES).map_err(|_| MalformedResponse)? {
        return Err(MalformedResponse);
    }
    let mut bytes =
        Vec::with_capacity(usize::try_from(artifact.byte_length).map_err(|_| MalformedResponse)?);
    artifact
        .try_reader()
        .and_then(|mut reader| reader.read_to_end(&mut bytes))
        .map_err(|_| MalformedResponse)?;
    let value = parse_strict_bounded_json(&bytes).map_err(|_| MalformedResponse)?;
    validate_schema_definition(definition, &value).map_err(|_| MalformedResponse)?;
    serde_json::from_value(value).map_err(|_| MalformedResponse)
}

fn response_binding(protocol: &str, adapter_kind: &str) -> ResponseBinding {
    ResponseBinding {
        protocol: ContractBinding {
            kind: protocol.to_owned(),
            version: wire::CONTRACT_VERSION.to_owned(),
            revision: "e02-draft-2".to_owned(),
            digest: ContentDigest::of_bytes(CONTRACT_BYTES),
        },
        adapter: ContractBinding {
            kind: adapter_kind.to_owned(),
            version: ADAPTER_VERSION.to_owned(),
            revision: env!("CARGO_PKG_VERSION").to_owned(),
            digest: ContentDigest::of_bytes(ADAPTER_BYTES),
        },
        exit_codes: ExitCodeBinding::Exact(BTreeSet::from([0])),
    }
}

fn require_reference_bytes<'a>(
    reference: &Value,
    expected_role: &str,
    proofs: &'a [ValidatedSharedReference],
) -> Result<&'a [u8], VerificationError> {
    let canonical = jcs_canonicalize(reference)?;
    proofs
        .iter()
        .filter_map(ValidatedSharedReference::artifact_proof)
        .find(|(candidate, role, _)| *candidate == canonical && *role == expected_role)
        .map(|(_, _, bytes)| bytes)
        .ok_or_else(|| {
            invalid_binding(
                "reference lacks an exact retained strict-reader proof",
                expected_role,
            )
        })
}

fn encoding_refusal(error: impl std::fmt::Display) -> VerificationError {
    invalid_binding(
        "typed execution value could not be encoded",
        &error.to_string(),
    )
}

fn invalid_binding(message: &str, value: &str) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::InvalidQualificationOracleBinding,
        message,
    )
    .with_context("value", value)
}
