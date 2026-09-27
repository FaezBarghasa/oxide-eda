---
okf_version: "0.2"
type: Module
title: filter_presets
description: Footprint selection-filter preset apply/capture helpers.
resource: crates/oxide-app/src/library/editor/footprint/filter_presets.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/filter_presets
language: rust
---

# filter_presets

Footprint selection-filter preset apply/capture helpers.

## Docstring

Footprint selection-filter preset apply/capture helpers.

Task 6 — parallel to the schematic's `CustomFilterPreset` flow
(`crate::app::handlers::active_bar::filter_controls`), but scoped
to the footprint editor's `SelectionFilterKind` categories and
backed by `FootprintFilterPreset` (Task 5).

## Relationships

| Type | Target |
|------|--------|
| related | [apply_preset](/crates/oxide-app/src/library/editor/footprint/filter_presets/apply_preset.md) |
| related | [capture_preset](/crates/oxide-app/src/library/editor/footprint/filter_presets/capture_preset.md) |
