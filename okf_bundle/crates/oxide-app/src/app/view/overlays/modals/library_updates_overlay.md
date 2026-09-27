---
okf_version: "0.2"
type: Function
title: library_updates_overlay
description: "\"Library Updates Available\" modal (Stage 16 §3.5). Opened on"
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/library_updates_overlay
language: rust
---

# library_updates_overlay

"Library Updates Available" modal (Stage 16 §3.5). Opened on

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn library_updates_overlay(&self) -> Option<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

"Library Updates Available" modal (Stage 16 §3.5). Opened on
schematic open under Team workflow mode when a placed Symbol's
`library_version` drifts from the source row's current version.

## Source
Lines 482–499 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
