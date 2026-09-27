---
okf_version: "0.2"
type: Module
title: placement
description: Footprint sketch updates — numeric placement-input buffer concern.
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement
language: rust
---

# placement

Footprint sketch updates — numeric placement-input buffer concern.

## Docstring

Footprint sketch updates — numeric placement-input buffer concern.

Carved out of the monolithic `sketch::apply` (ADR-0001 D1/D2). `apply`
is a thin router; each variant delegates to one named per-action fn
below (object→action, ADR-0001 D2).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/apply.md) |
| related | [input_char](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_char.md) |
| related | [input_backspace](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_backspace.md) |
| related | [input_enter](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_enter.md) |
| related | [input_escape](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_escape.md) |
| related | [input_tab](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_tab.md) |
