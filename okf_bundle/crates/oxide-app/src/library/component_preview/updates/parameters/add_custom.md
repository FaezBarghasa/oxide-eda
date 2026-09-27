---
okf_version: "0.2"
type: Function
title: add_custom
description: Add a custom parameter row with an empty value of the chosen kind.
resource: crates/oxide-app/src/library/component_preview/updates/parameters.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/parameters/add_custom
language: rust
---

# add_custom

Add a custom parameter row with an empty value of the chosen kind.

## Signature

```rust
pub(super) fn add_custom(state: &mut ComponentPreviewState, name: String, kind: ParamKindMsg)
```

## Visibility

- `pub(super)`

## Docstring

Add a custom parameter row with an empty value of the chosen kind.
Ignores a blank name.

## Source
Lines 77–92 in `crates/oxide-app/src/library/component_preview/updates/parameters.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameters](/crates/oxide-app/src/library/component_preview/updates/parameters.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
