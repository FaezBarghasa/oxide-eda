---
okf_version: "0.2"
type: Function
title: placement_fields
description: v0.14-footprint — the ordered Tab-cycle of typed dimension
resource: crates/oxide-app/src/library/editor/footprint/state/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/placement/placement_fields
language: rust
---

# placement_fields

v0.14-footprint — the ordered Tab-cycle of typed dimension

## Signature

```rust
impl PlacementInputKind { pub fn placement_fields(tool: SketchTool, pending: &ToolPending) -> Vec<Self> }
```

## Visibility

- `pub`

## Docstring

v0.14-footprint — the ordered Tab-cycle of typed dimension
fields for a tool's current gesture stage. More than one element
for tools whose shape is defined by several dimensions at the
SAME commit click (Line len/angle, Rectangle w/h, Rounded-Rect
w/h/radius); single-element for radius/sweep/distance tools;
empty for tools that take no typed dimensions. Single source of
truth for `from_active_tool` and the Tab field-cycle.

## Source
Lines 61–79 in `crates/oxide-app/src/library/editor/footprint/state/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/library/editor/footprint/state/placement.md) |
