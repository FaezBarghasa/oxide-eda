---
okf_version: "0.2"
type: Function
title: clear_overrides
description: "Clear every override, reverting to the default number-based match,"
resource: crates/oxide-app/src/library/component_preview/updates/pin_map.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/pin_map/clear_overrides
language: rust
---

# clear_overrides

Clear every override, reverting to the default number-based match,

## Signature

```rust
pub(super) fn clear_overrides(state: &mut ComponentPreviewState)
```

## Visibility

- `pub(super)`

## Docstring

Clear every override, reverting to the default number-based match,
and collapse the inline override editor.

## Source
Lines 12–17 in `crates/oxide-app/src/library/component_preview/updates/pin_map.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_map](/crates/oxide-app/src/library/component_preview/updates/pin_map.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
