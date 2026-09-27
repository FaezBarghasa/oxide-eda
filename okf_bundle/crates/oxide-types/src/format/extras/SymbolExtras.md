---
okf_version: "0.2"
type: Class
title: SymbolExtras
description: "Per-symbol auxiliary fields that don't fit into [`SchComponentRow`]."
resource: crates/oxide-types/src/format/extras.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/extras/SymbolExtras
language: rust
---

# SymbolExtras

Per-symbol auxiliary fields that don't fit into [`SchComponentRow`].

## Signature

```rust
pub(in crate::format) struct SymbolExtras
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub(in crate::format)`

## Docstring

Per-symbol auxiliary fields that don't fit into [`SchComponentRow`].
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `footprint`
- `datasheet`
- `mirror_x`
- `mirror_y`
- `unit`
- `is_power`
- `fields_autoplaced`
- `fields_user_placed`
- `dnp`
- `in_bom`
- `on_board`
- `exclude_from_sim`
- `locked`
- `fields`
- `custom_properties`
- `pin_uuids`
- `instances`
- `ref_text`
- `val_text`

## Source
Lines 55–94 in `crates/oxide-types/src/format/extras.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [extras](/crates/oxide-types/src/format/extras.md) |
