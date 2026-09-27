---
okf_version: "0.2"
type: Class
title: FootprintFilterPreset
description: A footprint-editor selection-filter preset. Parallel to
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/FootprintFilterPreset
language: rust
---

# FootprintFilterPreset

A footprint-editor selection-filter preset. Parallel to

## Signature

```rust
pub struct FootprintFilterPreset
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

A footprint-editor selection-filter preset. Parallel to
`CustomFilterPreset` but keyed on `SelectionFilterKind` (footprint
categories) instead of the schematic `SelectionFilter`. Persisted to
`prefs.json` under `footprint_filter_presets` (Task 6).
[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `name`
- `kinds`

## Source
Lines 206–209 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
