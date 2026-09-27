---
okf_version: "0.2"
type: Function
title: return_path_split_crossing
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/return_path_split_crossing
language: rust
---

# return_path_split_crossing

## Signature

```rust
impl RuleViolation { pub fn return_path_split_crossing(
        net: &str,
        net_class: &str,
        plane_net: &str,
        location: Option<(f64, f64)>,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 148–165 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
| calls | [NetClass](/crates/oxide-types/src/net/NetClass.md) |
