---
okf_version: "0.2"
type: Function
title: from_active_tool
description: v0.24 Track D — the default focused field when a gesture stage
resource: crates/oxide-app/src/library/editor/footprint/state/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/placement/from_active_tool_1
language: rust
---

# from_active_tool

v0.24 Track D — the default focused field when a gesture stage

## Signature

```rust
pub fn from_active_tool(tool: SketchTool, pending: &ToolPending) -> Option<Self>
```

## Visibility

- `pub`

## Docstring

v0.24 Track D — the default focused field when a gesture stage
opens: the first of `placement_fields`. Drives the canvas
keyboard guard and the kind minted for the first typed digit.

## Source
Lines 84–86 in `crates/oxide-app/src/library/editor/footprint/state/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/library/editor/footprint/state/placement.md) |
