---
okf_version: "0.2"
type: Function
title: input_tab
description: v0.14-footprint — cycle the focused dimension field to the next one in
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_tab
language: rust
---

# input_tab

v0.14-footprint — cycle the focused dimension field to the next one in

## Signature

```rust
fn input_tab(editor: &mut crate::app::FootprintEditorState)
```

## Docstring

v0.14-footprint — cycle the focused dimension field to the next one in
the active tool's Tab order (Line len→angle, Rectangle w→h,
Rounded-Rect w→h→radius→w…). The focused field lives in
`placement_input`; the rest park in `placement_input_others`, each
keeping its own typed digits. The canvas only emits this while a
buffer is open on a multi-field tool, but the dispatcher stays
defensive and no-ops unless the active tool exposes ≥2 fields.

## Source
Lines 119–159 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/apply.md) |
