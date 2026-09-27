---
okf_version: "0.2"
type: Function
title: add_new_sibling
description: v0.18.7 — append a fresh empty footprint to the envelope
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/add_new_sibling
language: rust
---

# add_new_sibling

v0.18.7 — append a fresh empty footprint to the envelope

## Signature

```rust
fn add_new_sibling(editor: &mut crate::app::FootprintEditorState)
```

## Docstring

v0.18.7 — append a fresh empty footprint to the envelope
and switch onto it. Names the new sibling `Footprint N`
where N counts existing siblings + 1; the user can rename
via the Properties panel.

## Source
Lines 41–51 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
