---
okf_version: "0.2"
type: Function
title: library_recovery_overlay
description: "Library recovery dialog (Stage 10). Surfaces missing-snxlib,"
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/library_recovery_overlay_1
language: rust
---

# library_recovery_overlay

Library recovery dialog (Stage 10). Surfaces missing-snxlib,

## Signature

```rust
pub(in crate::app::view) fn library_recovery_overlay(&self) -> Option<Element<'_, Message>>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Library recovery dialog (Stage 10). Surfaces missing-snxlib,
missing-.git, and broken primitive bindings as user-facing modals
instead of silent log lines.

## Source
Lines 446–463 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
