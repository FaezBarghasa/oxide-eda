---
okf_version: "0.2"
type: Function
title: clearance_violation
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/clearance_violation_1
language: rust
---

# clearance_violation

## Signature

```rust
pub fn clearance_violation(
        net_a: &str,
        net_b: &str,
        scope: RuleScope,
        min_clearance_microns: i64,
        actual_distance_microns: i64,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 62–82 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
