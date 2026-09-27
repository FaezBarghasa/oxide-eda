---
okf_version: "0.2"
type: Function
title: validate_and_enrich
description: "Validate and enrich AI circuit intent with real, verified parts."
resource: crates/oxide-ai/src/validator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ai"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-ai/src/validator/validate_and_enrich
language: rust
---

# validate_and_enrich

Validate and enrich AI circuit intent with real, verified parts.

## Signature

```rust
pub fn validate_and_enrich(
    intent: &mut CircuitIntent,
    service: &dyn PartLookupService,
) -> Result<(), ValidationError>
```

## Visibility

- `pub`

## Docstring

Validate and enrich AI circuit intent with real, verified parts.

## Source
Lines 72–91 in `crates/oxide-ai/src/validator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [validator](/crates/oxide-ai/src/validator.md) |
| called_by | [test_real_part_validator_enrichment](/crates/oxide-ai/tests/ai_tests/test_real_part_validator_enrichment.md) |
| called_by | [test_unmatched_hallucinated_part_fails_validation](/crates/oxide-ai/tests/ai_tests/test_unmatched_hallucinated_part_fails_validation.md) |
