---
okf_version: "0.2"
type: Function
title: parts_mut
description: Split-borrow accessor returning mutable references to
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/parts_mut_1
language: rust
---

# parts_mut

Split-borrow accessor returning mutable references to

## Signature

```rust
pub fn parts_mut(
        &mut self,
    ) -> (
        &mut crate::library::editor::footprint::state::FootprintEditorState,
        &mut Footprint,
    )
```

## Visibility

- `pub`

## Docstring

Split-borrow accessor returning mutable references to
`state` and the active primitive simultaneously. The two
fields are disjoint (`state` lives next to `file`), but
`&mut self`-shaped methods can't express that — calling
`editor.primitive_mut()` after `&mut editor.state` trips the
borrow checker. Destructuring `Self` makes disjointness
explicit. Callers passing both halves into a helper like
`apply_sketch_edit_with_warnings` reach for this.

## Source
Lines 606–620 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
