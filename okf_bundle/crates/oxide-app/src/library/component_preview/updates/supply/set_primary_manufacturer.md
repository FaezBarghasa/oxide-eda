---
okf_version: "0.2"
type: Function
title: set_primary_manufacturer
description: "Set the primary part's manufacturer name."
resource: crates/oxide-app/src/library/component_preview/updates/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/supply/set_primary_manufacturer
language: rust
---

# set_primary_manufacturer

Set the primary part's manufacturer name.

## Signature

```rust
pub(super) fn set_primary_manufacturer(state: &mut ComponentPreviewState, value: String)
```

## Visibility

- `pub(super)`

## Docstring

Set the primary part's manufacturer name.

## Source
Lines 11–14 in `crates/oxide-app/src/library/component_preview/updates/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/component_preview/updates/supply.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
