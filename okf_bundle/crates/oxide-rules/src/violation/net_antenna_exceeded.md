---
okf_version: "0.2"
type: Function
title: net_antenna_exceeded
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/net_antenna_exceeded
language: rust
---

# net_antenna_exceeded

## Signature

```rust
impl RuleViolation { pub fn net_antenna_exceeded(
        net: &str,
        scope: RuleScope,
        max_stub_microns: i64,
        actual_stub_microns: i64,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 127–146 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
