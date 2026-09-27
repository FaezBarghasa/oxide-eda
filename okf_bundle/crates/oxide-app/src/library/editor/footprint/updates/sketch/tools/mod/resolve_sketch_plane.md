---
okf_version: "0.2"
type: Function
title: resolve_sketch_plane
description: v0.22 Phase A1 — ensure the sketch has at least one plane so a fresh
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_sketch_plane
language: rust
---

# resolve_sketch_plane

v0.22 Phase A1 — ensure the sketch has at least one plane so a fresh

## Signature

```rust
fn resolve_sketch_plane(editor: &mut crate::app::FootprintEditorState) -> PlaneId
```

## Docstring

v0.22 Phase A1 — ensure the sketch has at least one plane so a fresh
click has somewhere to mint its Point.

## Source
Lines 287–304 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
| called_by | [handle_tool_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/handle_tool_click.md) |
