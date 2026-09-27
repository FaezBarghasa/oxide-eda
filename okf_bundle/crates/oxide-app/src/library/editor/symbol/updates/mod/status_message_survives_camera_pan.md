---
okf_version: "0.2"
type: Function
title: status_message_survives_camera_pan
description: Continuous/chrome-only messages (camera pan here) must NOT
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/status_message_survives_camera_pan
language: rust
---

# status_message_survives_camera_pan

Continuous/chrome-only messages (camera pan here) must NOT

## Signature

```rust
fn status_message_survives_camera_pan()
```

## Decorators

- `test`

## Docstring

Continuous/chrome-only messages (camera pan here) must NOT
clear a just-set status message before the user can read it.
[test]

## Source
Lines 633–640 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
