---
okf_version: "0.2"
type: Function
title: validate_antenna_length
description: "Validate that a net's dangling trace stub length does not exceed antenna threshold."
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/validate_antenna_length_1
language: rust
---

# validate_antenna_length

Validate that a net's dangling trace stub length does not exceed antenna threshold.

## Signature

```rust
pub fn validate_antenna_length(
        &self,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
        actual_stub_length: Microns,
    ) -> Result<(), RuleViolation>
```

## Decorators

- `allow(clippy::result_large_err)`

## Visibility

- `pub`

## Docstring

Validate that a net's dangling trace stub length does not exceed antenna threshold.
[allow(clippy::result_large_err)]

## Source
Lines 349–367 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
