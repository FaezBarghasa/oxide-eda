---
okf_version: "0.2"
type: Function
title: validate_clearance
description: Validate electrical clearance distance between two nets or objects.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/validate_clearance
language: rust
---

# validate_clearance

Validate electrical clearance distance between two nets or objects.

## Signature

```rust
impl ConstraintManager { pub fn validate_clearance(
        &self,
        net_a: &str,
        class_a: Option<&str>,
        net_b: &str,
        class_b: Option<&str>,
        room: Option<&str>,
        actual_distance: Microns,
    ) -> Result<(), RuleViolation> }
```

## Visibility

- `pub`

## Docstring

Validate electrical clearance distance between two nets or objects.
[allow(clippy::result_large_err)]

## Source
Lines 152–189 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
