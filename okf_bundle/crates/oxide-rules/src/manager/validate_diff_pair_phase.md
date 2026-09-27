---
okf_version: "0.2"
type: Function
title: validate_diff_pair_phase
description: Validate differential pair intra-pair phase skew or inter-pair bus skew.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/validate_diff_pair_phase
language: rust
---

# validate_diff_pair_phase

Validate differential pair intra-pair phase skew or inter-pair bus skew.

## Signature

```rust
impl ConstraintManager { pub fn validate_diff_pair_phase(
        &self,
        net_class: &str,
        pair_or_bus: &str,
        actual_skew: Microns,
        is_intra_pair: bool,
    ) -> Result<(), RuleViolation> }
```

## Visibility

- `pub`

## Docstring

Validate differential pair intra-pair phase skew or inter-pair bus skew.
[allow(clippy::result_large_err)]

## Source
Lines 510–534 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
