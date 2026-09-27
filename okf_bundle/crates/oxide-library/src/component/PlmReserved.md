---
okf_version: "0.2"
type: Class
title: PlmReserved
description: PLM-reserved fields. Inert until v3.0.
resource: crates/oxide-library/src/component.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/component/PlmReserved
language: rust
---

# PlmReserved

PLM-reserved fields. Inert until v3.0.

## Signature

```rust
pub struct PlmReserved
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

PLM-reserved fields. Inert until v3.0.

**TSV persistence note:** as of v0.9, `PlmReserved` is dropped at write
time — only `PlmReserved::default()` round-trips through `tables::write_table`.
Non-default payloads cause `LibraryError::Backend("...")`. v3.0 will add
dedicated columns and full round-trip.
[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `plm_part_id`
- `eco_refs`
- `compliance`

## Source
Lines 79–86 in `crates/oxide-library/src/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/oxide-library/src/component.md) |
