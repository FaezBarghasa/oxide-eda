---
okf_version: "0.2"
type: Function
title: shortcut_label
description: "Resolve a keymap command id to its label under the active profile,"
resource: crates/oxide-app/src/library/editor/footprint/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/context_menu/shortcut_label
language: rust
---

# shortcut_label

Resolve a keymap command id to its label under the active profile,

## Signature

```rust
fn shortcut_label(keymap: &CompiledKeymap, command_id: &str, fallback: &str) -> String
```

## Docstring

Resolve a keymap command id to its label under the active profile,
falling back to `fallback` when the command is unbound.

## Source
Lines 345–350 in `crates/oxide-app/src/library/editor/footprint/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/context_menu.md) |
| called_by | [view_context_menu](/crates/oxide-app/src/library/editor/footprint/context_menu/view_context_menu.md) |
