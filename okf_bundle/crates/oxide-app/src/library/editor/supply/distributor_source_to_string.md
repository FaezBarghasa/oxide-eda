---
okf_version: "0.2"
type: Function
title: distributor_source_to_string
description: "Convert a picked `DistributorSource` to the canonical string written"
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/distributor_source_to_string
language: rust
---

# distributor_source_to_string

Convert a picked `DistributorSource` to the canonical string written

## Signature

```rust
pub(crate) fn distributor_source_to_string(s: DistributorSource) -> String
```

## Visibility

- `pub(crate)`

## Docstring

Convert a picked `DistributorSource` to the canonical string written
onto `DistributorListing.distributor`. Used by the dispatcher.

## Source
Lines 113–115 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| calls | [distributor_label](/crates/oxide-app/src/library/editor/supply/distributor_label.md) |
| called_by | [set_listing_distributor](/crates/oxide-app/src/library/component_preview/updates/supply/set_listing_distributor.md) |
