---
okf_version: "0.2"
type: Function
title: set_listing_sku
description: "Set listing `idx`'s distributor SKU."
resource: crates/oxide-app/src/library/component_preview/updates/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/supply/set_listing_sku
language: rust
---

# set_listing_sku

Set listing `idx`'s distributor SKU.

## Signature

```rust
pub(super) fn set_listing_sku(state: &mut ComponentPreviewState, idx: usize, value: String)
```

## Visibility

- `pub(super)`

## Docstring

Set listing `idx`'s distributor SKU.

## Source
Lines 129–134 in `crates/oxide-app/src/library/component_preview/updates/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/component_preview/updates/supply.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
