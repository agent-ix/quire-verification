---
id: interface-001
title: "Complete-V1 verification method provider"
type: interface
---
# [interface-001] Complete-V1 verification method provider

## Contract

```yaml
name: VerificationMethodProvider
contract_versions:
  plan: quire.verification-plan/v1
  result: quire.method-result/v1
associated_types:
  - VerificationPlan
  - PlanNode
  - MethodRequest
  - MethodResult
  - MethodWitness
operations:
  - name: capabilities
    inputs: [provider manifest, ecosystem lock]
    output: exact supported methods, domains, bounds, and limitations
    semantics: declares capabilities without widening clause admission or result strength
  - name: prepare
    inputs: [linked plan node, checked package, semantic subject, ecosystem lock]
    output: MethodRequest or typed unsupported/refused disposition
    semantics: resolves every subject, assumption, domain, bound, option, and provider pin before execution
  - name: execute
    inputs: [MethodRequest]
    output: MethodResult
    semantics: returns one immutable typed outcome with exact method, implementation, run, coverage, provenance, and witness data
  - name: replay
    inputs: [MethodRequest, MethodWitness, exact provider and subject revisions]
    output: replay MethodResult
    semantics: independently checks the same scoped witness without mutating the original result
outcomes:
  semantic: [proved, refuted, tested, observed]
  non_conclusive: [inconclusive, unsupported, incomplete, failed]
  rule: method failure or non-conclusion is never converted into a semantic verdict
invariants:
  - every plan node receives exactly one terminal disposition
  - semantic bounds and provider work limits are distinct
  - plans select methods but cannot alter source admission
  - candidate discovery and independent validation are separate runs
  - result identity binds the exact request, provider, implementation, toolchain, options, subject, assumptions, bounds, and content
```

