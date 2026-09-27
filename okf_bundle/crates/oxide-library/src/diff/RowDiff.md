---
okf_version: "0.2"
type: Class
title: RowDiff
description: Field-level changed flags + grouped detail. Mirrors plan §6 step 1.7.
resource: crates/oxide-library/src/diff.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/diff/RowDiff
language: rust
---

# RowDiff

Field-level changed flags + grouped detail. Mirrors plan §6 step 1.7.

## Signature

```rust
pub struct RowDiff
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq)`

## Visibility

- `pub`

## Docstring

Field-level changed flags + grouped detail. Mirrors plan §6 step 1.7.

The eleven boolean fields are the dimensions the auto-bump heuristic
inspects; the trailing detail rows are populated only for the dimensions
that actually changed.
[derive(Clone, Debug, Default, PartialEq)]

## Methods

- `symbol_changed`
- `footprint_changed`
- `sim_changed`
- `pin_map_changed`
- `params_changed`
- `mpn_changed`
- `alternates_changed`
- `supply_changed`
- `datasheet_changed`
- `state_changed`
- `plm_changed`
- `parameters`
- `pin_map`
- `alternates_detail`
- `supply_detail`
- `lifecycle_detail`

## Source
Lines 28–47 in `crates/oxide-library/src/diff.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff](/crates/oxide-library/src/diff.md) |
