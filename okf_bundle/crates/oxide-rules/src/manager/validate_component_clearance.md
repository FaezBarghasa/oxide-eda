---
okf_version: "0.2"
type: Function
title: validate_component_clearance
description: Validate 3D component horizontal (X/Y) or vertical (Z) clearance.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/validate_component_clearance
language: rust
---

# validate_component_clearance

Validate 3D component horizontal (X/Y) or vertical (Z) clearance.

## Signature

```rust
impl ConstraintManager { pub fn validate_component_clearance(
        &self,
        designator_a: &str,
        designator_b: &str,
        room: Option<&str>,
        actual_distance: Microns,
        is_vertical: bool,
    ) -> Result<(), RuleViolation> }
```

## Visibility

- `pub`

## Docstring

Validate 3D component horizontal (X/Y) or vertical (Z) clearance.
[allow(clippy::result_large_err)]

## Source
Lines 441–484 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
