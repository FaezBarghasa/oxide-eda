---
okf_version: "0.2"
type: Function
title: add_override
description: "Commit an override for `pin`: an empty pad clears it, otherwise the"
resource: crates/oxide-app/src/library/component_preview/updates/pin_map.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/pin_map/add_override
language: rust
---

# add_override

Commit an override for `pin`: an empty pad clears it, otherwise the

## Signature

```rust
pub(super) fn add_override(state: &mut ComponentPreviewState, pin: String, pad: String)
```

## Visibility

- `pub(super)`

## Docstring

Commit an override for `pin`: an empty pad clears it, otherwise the
existing entry is updated in place or a new one is pushed. Collapses
the inline editor afterwards.

## Source
Lines 53–76 in `crates/oxide-app/src/library/component_preview/updates/pin_map.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_map](/crates/oxide-app/src/library/component_preview/updates/pin_map.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
