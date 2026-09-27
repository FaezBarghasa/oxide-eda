---
okf_version: "0.2"
type: Function
title: validate_solder_mask_sliver
description: Validate that a solder mask bridge/sliver meets the minimum width requirement.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/validate_solder_mask_sliver_1
language: rust
---

# validate_solder_mask_sliver

Validate that a solder mask bridge/sliver meets the minimum width requirement.

## Signature

```rust
pub fn validate_solder_mask_sliver(
        &self,
        object_id: &str,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
        actual_sliver: Microns,
    ) -> Result<(), RuleViolation>
```

## Decorators

- `allow(clippy::result_large_err)`

## Visibility

- `pub`

## Docstring

Validate that a solder mask bridge/sliver meets the minimum width requirement.
[allow(clippy::result_large_err)]

## Source
Lines 301–320 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
