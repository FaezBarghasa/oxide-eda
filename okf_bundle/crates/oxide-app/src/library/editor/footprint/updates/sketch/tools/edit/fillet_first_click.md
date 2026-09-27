---
okf_version: "0.2"
type: Function
title: fillet_first_click
description: First click — pick the first Line.
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_first_click
language: rust
---

# fillet_first_click

First click — pick the first Line.

## Signature

```rust
fn fillet_first_click(editor: &mut crate::app::FootprintEditorState, click_xy: (f64, f64)
```

## Docstring

First click — pick the first Line.

## Source
Lines 115–134 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.md) |
| calls | [pick_line_at](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_at.md) |
| called_by | [fillet](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet.md) |
