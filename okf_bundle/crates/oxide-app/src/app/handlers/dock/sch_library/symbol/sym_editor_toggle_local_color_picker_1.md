---
okf_version: "0.2"
type: Function
title: sym_editor_toggle_local_color_picker
description: "Toggle a local-colour slot's picker open / closed. Opening it"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_toggle_local_color_picker_1
language: rust
---

# sym_editor_toggle_local_color_picker

Toggle a local-colour slot's picker open / closed. Opening it

## Signature

```rust
pub(super) fn sym_editor_toggle_local_color_picker(
        &mut self,
        slot: crate::app::LocalColorSlot,
    ) -> bool
```

## Visibility

- `pub(super)`

## Docstring

Toggle a local-colour slot's picker open / closed. Opening it
closes any graphic-fill picker. No dirty.

## Source
Lines 190–209 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
