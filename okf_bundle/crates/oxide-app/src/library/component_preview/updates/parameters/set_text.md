---
okf_version: "0.2"
type: Function
title: set_text
description: "Set a text parameter's value directly. Ignores an empty name."
resource: crates/oxide-app/src/library/component_preview/updates/parameters.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/parameters/set_text
language: rust
---

# set_text

Set a text parameter's value directly. Ignores an empty name.

## Signature

```rust
pub(super) fn set_text(state: &mut ComponentPreviewState, name: String, value: String)
```

## Visibility

- `pub(super)`

## Docstring

Set a text parameter's value directly. Ignores an empty name.

## Source
Lines 12–20 in `crates/oxide-app/src/library/component_preview/updates/parameters.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameters](/crates/oxide-app/src/library/component_preview/updates/parameters.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
| called_by | [upload](/crates/oxide-gfx/src/pipeline/text/upload.md) |
