---
okf_version: "0.2"
type: Function
title: delete_confirm_overlay
description: Delete Selected confirm modal (Deliverable D). First browser with
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/delete_confirm_overlay
language: rust
---

# delete_confirm_overlay

Delete Selected confirm modal (Deliverable D). First browser with

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn delete_confirm_overlay(&self) -> Option<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Delete Selected confirm modal (Deliverable D). First browser with
a pending confirm wins.

## Source
Lines 327–352 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
| calls | [view_delete_confirm](/crates/oxide-app/src/library/edit_row_modal/view_delete_confirm.md) |
