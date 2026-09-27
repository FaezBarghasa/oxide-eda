---
okf_version: "0.2"
type: Module
title: ui
description: "Footprint sketch updates — selection & tool/mode UI concern."
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui
language: rust
---

# ui

Footprint sketch updates — selection & tool/mode UI concern.

## Docstring

Footprint sketch updates — selection & tool/mode UI concern.

Carved out of the monolithic `sketch::apply` (ADR-0001 D1/D2). `apply`
is a thin router; each variant delegates to one named per-action fn
below (object→action, ADR-0001 D2).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/apply.md) |
| related | [select_many](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/select_many.md) |
| related | [set_tool](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/set_tool.md) |
| related | [toggle_construction](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/toggle_construction.md) |
| related | [toggle_centerline](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/toggle_centerline.md) |
| related | [tool_escape](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/tool_escape.md) |
| related | [select](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/select.md) |
| related | [dimension_input](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/dimension_input.md) |
