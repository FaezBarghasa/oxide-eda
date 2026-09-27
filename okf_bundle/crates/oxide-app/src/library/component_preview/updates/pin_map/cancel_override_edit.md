---
okf_version: "0.2"
type: Function
title: cancel_override_edit
description: Discard the edit buffer and collapse the inline override editor
resource: crates/oxide-app/src/library/component_preview/updates/pin_map.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/pin_map/cancel_override_edit
language: rust
---

# cancel_override_edit

Discard the edit buffer and collapse the inline override editor

## Signature

```rust
pub(super) fn cancel_override_edit(state: &mut ComponentPreviewState)
```

## Visibility

- `pub(super)`

## Docstring

Discard the edit buffer and collapse the inline override editor
without touching the stored overrides.

## Source
Lines 80–83 in `crates/oxide-app/src/library/component_preview/updates/pin_map.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_map](/crates/oxide-app/src/library/component_preview/updates/pin_map.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
