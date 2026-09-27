---
okf_version: "0.2"
type: Function
title: remove_override
description: "Remove `pin`'s override entry and collapse the inline editor."
resource: crates/oxide-app/src/library/component_preview/updates/pin_map.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/pin_map/remove_override
language: rust
---

# remove_override

Remove `pin`'s override entry and collapse the inline editor.

## Signature

```rust
pub(super) fn remove_override(state: &mut ComponentPreviewState, pin: String)
```

## Visibility

- `pub(super)`

## Docstring

Remove `pin`'s override entry and collapse the inline editor.

## Source
Lines 86–94 in `crates/oxide-app/src/library/component_preview/updates/pin_map.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_map](/crates/oxide-app/src/library/component_preview/updates/pin_map.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
