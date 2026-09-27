---
okf_version: "0.2"
type: Function
title: width_too_small
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/width_too_small
language: rust
---

# width_too_small

## Signature

```rust
impl RuleViolation { pub fn width_too_small(
        net: &str,
        scope: RuleScope,
        min_width_microns: i64,
        actual_width_microns: i64,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 41–60 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
