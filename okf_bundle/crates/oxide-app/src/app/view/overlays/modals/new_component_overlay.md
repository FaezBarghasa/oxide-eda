---
okf_version: "0.2"
type: Function
title: new_component_overlay
description: "New Component modal was removed (v0.13); \"Add Component\" now"
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/new_component_overlay
language: rust
---

# new_component_overlay

New Component modal was removed (v0.13); "Add Component" now

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn new_component_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

New Component modal was removed (v0.13); "Add Component" now
appends a draft row directly to the Library Browser table, so this
overlay deliberately contributes no layers.

It used to compute the per-library class list here purely to keep
that code exercised; the same computation lives in
`library/browser/sidebar.rs` and `dispatch/library/browser/classes.rs`,
which are the real consumers, so the copy was dropped. Prune this
function together with the `library.new_component` state + messages
once the append-row-direct dispatcher migration lands.

## Source
Lines 262–264 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
