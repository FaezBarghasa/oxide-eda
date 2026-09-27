---
okf_version: "0.2"
type: Function
title: silkscreen_clearance_violation
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/silkscreen_clearance_violation
language: rust
---

# silkscreen_clearance_violation

## Signature

```rust
impl RuleViolation { pub fn silkscreen_clearance_violation(
        object_a: &str,
        object_b: &str,
        scope: RuleScope,
        min_clearance_microns: i64,
        actual_distance_microns: i64,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 105–125 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
