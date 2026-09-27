---
okf_version: "0.2"
type: Function
title: phase_skew_violation
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/phase_skew_violation_1
language: rust
---

# phase_skew_violation

## Signature

```rust
pub fn phase_skew_violation(
        net_class: &str,
        pair_or_bus: &str,
        max_skew_microns: i64,
        actual_skew_microns: i64,
        is_intra_pair: bool,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 211–232 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
| calls | [NetClass](/crates/oxide-types/src/net/NetClass.md) |
