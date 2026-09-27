---
okf_version: "0.2"
type: Class
title: CustomFilterPreset
description: A user-defined named selection-filter preset. Persisted to
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/CustomFilterPreset
language: rust
---

# CustomFilterPreset

A user-defined named selection-filter preset. Persisted to

## Signature

```rust
pub struct CustomFilterPreset
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

A user-defined named selection-filter preset. Persisted to
`~/.config/oxide/prefs.json` under `custom_filter_presets` and
surfaced as a shortcut button in the Active Bar's filter dropdown.
[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `name`
- `filters`

## Source
Lines 174–179 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
