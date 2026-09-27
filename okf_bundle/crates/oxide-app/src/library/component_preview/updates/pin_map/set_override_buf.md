---
okf_version: "0.2"
type: Function
title: set_override_buf
description: "Live-update the override edit buffer, but only while `pin`'s row is"
resource: crates/oxide-app/src/library/component_preview/updates/pin_map.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/pin_map/set_override_buf
language: rust
---

# set_override_buf

Live-update the override edit buffer, but only while `pin`'s row is

## Signature

```rust
pub(super) fn set_override_buf(state: &mut ComponentPreviewState, pin: String, value: String)
```

## Visibility

- `pub(super)`

## Docstring

Live-update the override edit buffer, but only while `pin`'s row is
the one currently expanded.

## Source
Lines 44–48 in `crates/oxide-app/src/library/component_preview/updates/pin_map.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_map](/crates/oxide-app/src/library/component_preview/updates/pin_map.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
