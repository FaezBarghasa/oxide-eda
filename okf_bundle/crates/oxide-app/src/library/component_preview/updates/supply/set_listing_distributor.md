---
okf_version: "0.2"
type: Function
title: set_listing_distributor
description: "Set listing `idx`'s distributor from a picked source."
resource: crates/oxide-app/src/library/component_preview/updates/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/supply/set_listing_distributor
language: rust
---

# set_listing_distributor

Set listing `idx`'s distributor from a picked source.

## Signature

```rust
pub(super) fn set_listing_distributor(
    state: &mut ComponentPreviewState,
    idx: usize,
    value: DistributorSource,
)
```

## Visibility

- `pub(super)`

## Docstring

Set listing `idx`'s distributor from a picked source.

## Source
Lines 117–126 in `crates/oxide-app/src/library/component_preview/updates/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/component_preview/updates/supply.md) |
| calls | [distributor_source_to_string](/crates/oxide-app/src/library/editor/supply/distributor_source_to_string.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
