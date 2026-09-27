---
okf_version: "0.2"
type: Function
title: align_entries
description: Align submenu — pairwise aligns + distribute (gated by selection
resource: crates/oxide-app/src/app/view/context_menu/submenu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/submenu/align_entries
language: rust
---

# align_entries

Align submenu — pairwise aligns + distribute (gated by selection

## Signature

```rust
pub(super) fn align_entries(tid: ThemeId, selected: usize) -> Vec<DropdownEntry<Message>>
```

## Visibility

- `pub(super)`

## Docstring

Align submenu — pairwise aligns + distribute (gated by selection
count) and the always-on Align To Grid.

## Source
Lines 127–190 in `crates/oxide-app/src/app/view/context_menu/submenu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [submenu](/crates/oxide-app/src/app/view/context_menu/submenu.md) |
| calls | [align_gate](/crates/oxide-app/src/app/view/context_menu/submenu/align_gate.md) |
| calls | [dd_kb](/crates/oxide-app/src/app/view/context_menu/items/dd_kb.md) |
| calls | [dd_disabled](/crates/oxide-app/src/app/view/context_menu/items/dd_disabled.md) |
| called_by | [view_context_submenu](/crates/oxide-app/src/app/view/context_menu/submenu/view_context_submenu.md) |
