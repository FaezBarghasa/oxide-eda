---
okf_version: "0.2"
type: Function
title: open_override_edit
description: "Expand the inline override editor for `pin`, seeding the edit buffer"
resource: crates/oxide-app/src/library/component_preview/updates/pin_map.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/pin_map/open_override_edit
language: rust
---

# open_override_edit

Expand the inline override editor for `pin`, seeding the edit buffer

## Signature

```rust
pub(super) fn open_override_edit(state: &mut ComponentPreviewState, pin: String)
```

## Visibility

- `pub(super)`

## Docstring

Expand the inline override editor for `pin`, seeding the edit buffer
with that pin's current pad number (empty when unset).

## Source
Lines 30–40 in `crates/oxide-app/src/library/component_preview/updates/pin_map.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_map](/crates/oxide-app/src/library/component_preview/updates/pin_map.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
