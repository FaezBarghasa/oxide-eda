---
okf_version: "0.2"
type: Function
title: add_listing
description: "Append a new, empty distributor listing."
resource: crates/oxide-app/src/library/component_preview/updates/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/supply/add_listing
language: rust
---

# add_listing

Append a new, empty distributor listing.

## Signature

```rust
pub(super) fn add_listing(state: &mut ComponentPreviewState)
```

## Visibility

- `pub(super)`

## Docstring

Append a new, empty distributor listing.

## Source
Lines 106–114 in `crates/oxide-app/src/library/component_preview/updates/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/component_preview/updates/supply.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
