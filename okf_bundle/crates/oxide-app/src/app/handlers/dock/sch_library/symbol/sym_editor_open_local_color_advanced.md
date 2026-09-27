---
okf_version: "0.2"
type: Function
title: sym_editor_open_local_color_advanced
description: "Expand a local-colour slot's picker into the HSV / RGB overlay."
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_open_local_color_advanced
language: rust
---

# sym_editor_open_local_color_advanced

Expand a local-colour slot's picker into the HSV / RGB overlay.

## Signature

```rust
impl Oxide { pub(super) fn sym_editor_open_local_color_advanced(
        &mut self,
        slot: crate::app::LocalColorSlot,
    ) -> bool }
```

## Visibility

- `pub(super)`

## Docstring

Expand a local-colour slot's picker into the HSV / RGB overlay.
No dirty.

## Source
Lines 213–227 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
