---
id: interface-002
title: "Complete-V1 qualification evidence authority"
type: interface
---
# [interface-002] Complete-V1 qualification evidence authority

## Contract

```yaml
name: QualificationEvidenceAuthority
contract_versions:
  ecosystem_lock: quire.ecosystem-lock/v1
  conformance_corpus: quire.conformance-corpus/v1
  evidence_intake: quire.evidence-intake/v1
  human_decision: quire.human-decision/v1
associated_types:
  - EcosystemLock
  - ConformanceCorpus
  - EvidenceGraph
  - EvidenceIntake
  - CounterexampleRevision
  - QualificationLedger
  - HumanDecision
operations:
  - name: link
    inputs: [exact typed subjects, relations, results, artifacts, provenance, limitations]
    output: EvidenceGraph or typed refusal
    semantics: creates scoped typed edges without substituting identity kinds or accepting circular self-evidence
  - name: revise_counterexample
    inputs: [parent revision, transformation, correspondence, replay result]
    output: immutable child revision or lineage refusal
    semantics: preserves the parent and records claim-relative domain and verdict equivalence
  - name: account
    inputs: [accepted 176-row inventory, qualification profile, corpus, ecosystem lock, evidence graph]
    output: canonical QualificationLedger
    semantics: emits one primary row per capability and independent claim-class vectors with explicit planned/unexecuted/passed/failed/unsupported/waived state
  - name: bind_decision
    inputs: [unchanged ledger scope, authenticated actor, authority, evidence set, disposition, rationale]
    output: immutable HumanDecision or typed refusal
    semantics: records eventual acceptance without making production ceremony a prerequisite for development execution
ownership:
  producer: quire-verification emits validated immutable records
  retention_and_sufficiency: Quoin
  integrated_execution: quire-research issue 78
  decision: authenticated human or explicitly authorized workflow
invariants:
  - exact source, package, model, binding, implementation, method, run, result, artifact, and decision identities remain distinct
  - all 176 accepted inventory capabilities occur exactly once as primary ledger rows
  - missing production evidence is explicit ledger state and never blocks development or local tests
  - redaction or loss narrows affected claims
  - a score, majority, waiver, self-reference, or cycle cannot promote acceptance
```

