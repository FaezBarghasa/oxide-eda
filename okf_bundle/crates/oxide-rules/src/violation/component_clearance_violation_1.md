---
okf_version: "0.2"
type: Function
title: component_clearance_violation
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/component_clearance_violation_1
language: rust
---

# component_clearance_violation

## Signature

```rust
pub fn component_clearance_violation(
        designator_a: &str,
        designator_b: &str,
        scope: RuleScope,
        min_clearance_microns: i64,
        actual_distance_microns: i64,
        is_vertical: bool,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 167–189 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
