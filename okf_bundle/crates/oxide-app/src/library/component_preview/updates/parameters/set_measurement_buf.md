---
okf_version: "0.2"
type: Function
title: set_measurement_buf
description: Live-update the edit buffer for a measurement parameter row.
resource: crates/oxide-app/src/library/component_preview/updates/parameters.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/parameters/set_measurement_buf
language: rust
---

# set_measurement_buf

Live-update the edit buffer for a measurement parameter row.

## Signature

```rust
pub(super) fn set_measurement_buf(state: &mut ComponentPreviewState, name: String, buf: String)
```

## Visibility

- `pub(super)`

## Docstring

Live-update the edit buffer for a measurement parameter row.

## Source
Lines 42–44 in `crates/oxide-app/src/library/component_preview/updates/parameters.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameters](/crates/oxide-app/src/library/component_preview/updates/parameters.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
