---
okf_version: "0.2"
type: Function
title: keymap_shortcut_label
description: Resolve a keymap command id to its display label under the active
resource: crates/oxide-app/src/app/view/context_menu/items.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/items/keymap_shortcut_label_1
language: rust
---

# keymap_shortcut_label

Resolve a keymap command id to its display label under the active

## Signature

```rust
pub(in crate::app::view) fn keymap_shortcut_label(
        &self,
        command_id: &str,
        fallback: &str,
    ) -> String
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Resolve a keymap command id to its display label under the active
profile, falling back to `fallback` when the command is unbound.

## Source
Lines 203–212 in `crates/oxide-app/src/app/view/context_menu/items.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [items](/crates/oxide-app/src/app/view/context_menu/items.md) |
