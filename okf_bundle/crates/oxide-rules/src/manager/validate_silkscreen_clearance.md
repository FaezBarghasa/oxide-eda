---
okf_version: "0.2"
type: Function
title: validate_silkscreen_clearance
description: Validate silkscreen clearance to solder mask or adjacent silkscreen.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/validate_silkscreen_clearance
language: rust
---

# validate_silkscreen_clearance

Validate silkscreen clearance to solder mask or adjacent silkscreen.

## Signature

```rust
impl ConstraintManager { pub fn validate_silkscreen_clearance(
        &self,
        object_a: &str,
        object_b: &str,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
        actual_distance: Microns,
    ) -> Result<(), RuleViolation> }
```

## Visibility

- `pub`

## Docstring

Validate silkscreen clearance to solder mask or adjacent silkscreen.
[allow(clippy::result_large_err)]

## Source
Lines 324–345 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
