---
okf_version: "0.2"
type: Function
title: edit_row_modal_overlay
description: F25 (2026-05-03) — Edit Component Details modal removed. Row click
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/edit_row_modal_overlay_1
language: rust
---

# edit_row_modal_overlay

F25 (2026-05-03) — Edit Component Details modal removed. Row click

## Signature

```rust
pub(in crate::app::view) fn edit_row_modal_overlay(&self) -> Vec<Element<'_, Message>>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

F25 (2026-05-03) — Edit Component Details modal removed. Row click
selects → Properties panel surfaces detail. Render branch retained
behind `EDIT_MODAL_ENABLED` for one release; prune the supporting
state + dispatchers in a follow-up cleanup pass.

## Source
Lines 270–323 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
