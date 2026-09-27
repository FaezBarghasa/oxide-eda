---
okf_version: "0.2"
type: Class
title: Symbol
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/Symbol
language: rust
---

# Symbol

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct Symbol
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `uuid`
- `lib_id`
- `reference`
- `value`
- `footprint`
- `datasheet`
- `position`
- `rotation`
- `mirror_x`
- `mirror_y`
- `unit`
- `is_power`
- `ref_text`
- `val_text`
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
- `library_id`
- `row_id`
- `library_version`

## Source
Lines 635–714 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
