---
id: TC-041
title: "Classify oracle independence and disagreement"
type: TC
relationships:
  - target: "ix://agent-ix/quire-verification/FR-024"
    type: "verifies"
---
# TC-041: Classify oracle independence and disagreement

## Description

Verify exact independent, shared, derived, unknown, and absent oracle cases and
ensure agreement and disagreement affect only the relation they actually evaluate.

## Test Procedure

Run bounded exact oracle fixtures over the same corpus inputs while varying
implementation origin, shared parser/generator ancestry, projection, expected
relation, limitations, and observation. Include agreement, disagreement, no-oracle,
changed-oracle, and self-generated expected-value cases.

## Expected Results

Each contribution retains the correct dependence class. Agreement supports only its
declared relation. Disagreement retains both observations and blocks acceptance.
No-oracle remains a gap, self-generated values are not independent, and any changed
identity, projection, relation, input, or limitation refuses reuse.

