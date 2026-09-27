---
okf_version: "0.2"
type: Function
title: set_bool
description: Toggle a boolean parameter.
resource: crates/oxide-app/src/library/component_preview/updates/parameters.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/parameters/set_bool
language: rust
---

# set_bool

Toggle a boolean parameter.

## Signature

```rust
pub(super) fn set_bool(state: &mut ComponentPreviewState, name: String, value: bool)
```

## Visibility

- `pub(super)`

## Docstring

Toggle a boolean parameter.

## Source
Lines 61–67 in `crates/oxide-app/src/library/component_preview/updates/parameters.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameters](/crates/oxide-app/src/library/component_preview/updates/parameters.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
