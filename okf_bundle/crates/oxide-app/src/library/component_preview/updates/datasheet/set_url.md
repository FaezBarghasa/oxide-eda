---
okf_version: "0.2"
type: Function
title: set_url
description: "Set the datasheet to a URL reference, clearing it to the default when"
resource: crates/oxide-app/src/library/component_preview/updates/datasheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:28:52Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/datasheet/set_url
language: rust
---

# set_url

Set the datasheet to a URL reference, clearing it to the default when

## Signature

```rust
pub(super) fn set_url(state: &mut ComponentPreviewState, url: String)
```

## Visibility

- `pub(super)`

## Docstring

Set the datasheet to a URL reference, clearing it to the default when
the field is emptied.

## Source
Lines 39–47 in `crates/oxide-app/src/library/component_preview/updates/datasheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [datasheet](/crates/oxide-app/src/library/component_preview/updates/datasheet.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
