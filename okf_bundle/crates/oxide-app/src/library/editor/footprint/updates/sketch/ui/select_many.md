---
okf_version: "0.2"
type: Function
title: select_many
description: v0.27 — Sketch-mode multi-select replacement. First entity is primary
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/select_many
language: rust
---

# select_many

v0.27 — Sketch-mode multi-select replacement. First entity is primary

## Signature

```rust
fn select_many(
    editor: &mut crate::app::FootprintEditorState,
    ids: Vec<oxide_sketch::id::SketchEntityId>,
)
```

## Docstring

v0.27 — Sketch-mode multi-select replacement. First entity is primary
(drives the inspector + DOF overlay focus); the second slots into the
secondary (used by the constraint submenu's "two entities" pairing);
the rest land in extras. Empty list deselects everything.

## Source
Lines 29–43 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/apply.md) |
