---
okf_version: "0.2"
type: Function
title: select_silk_f
description: v0.18.18 — silk-front graphic selection. Clears
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/select_silk_f
language: rust
---

# select_silk_f

v0.18.18 — silk-front graphic selection. Clears

## Signature

```rust
fn select_silk_f(editor: &mut crate::app::FootprintEditorState, sel: Option<usize>)
```

## Docstring

v0.18.18 — silk-front graphic selection. Clears
selected_pad symmetrically so the Properties panel
doesn't try to render two selection-specific bodies at
once.

## Source
Lines 82–88 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |
