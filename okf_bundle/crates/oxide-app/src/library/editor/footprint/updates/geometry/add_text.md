---
okf_version: "0.2"
type: Function
title: add_text
description: v0.18.15 — Place String tool. Appends a silk-layer text
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/add_text
language: rust
---

# add_text

v0.18.15 — Place String tool. Appends a silk-layer text

## Signature

```rust
fn add_text(editor: &mut crate::app::FootprintEditorState, x_mm: f64, y_mm: f64)
```

## Docstring

v0.18.15 — Place String tool. Appends a silk-layer text
label `FpGraphic { kind: Text { position, content: "TEXT",
size: 1.0 }, stroke_width: 0.0 }` to the active footprint's
`silk_f`. The user edits the content via the Properties
panel later (Properties wiring is queued).

## Source
Lines 240–256 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
| called_by | [add_or_update](/crates/oxide-library/src/search_index/add_or_update.md) |
