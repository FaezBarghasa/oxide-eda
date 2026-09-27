---
okf_version: "0.2"
type: Function
title: refresh_components
description: Refresh the cached table contents for a library — re-reads every
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/refresh_components
language: rust
---

# refresh_components

Refresh the cached table contents for a library — re-reads every

## Signature

```rust
impl LibraryState { pub fn refresh_components(&mut self, root: &Path) -> Result<(), LibraryError> }
```

## Visibility

- `pub`

## Docstring

Refresh the cached table contents for a library — re-reads every
TSV via the mounted adapter's `list_tables` + `read_table`.

Returns `LibraryError::NotFound` when the library at `root`
isn't mounted, otherwise the underlying adapter error. The
cached `Vec<ComponentSummary>` is rebuilt alongside `tables`
so the picker (summary tier) and the panel grid (row tier)
stay coherent.

## Source
Lines 196–264 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
| calls | [apply_primitive_listing](/crates/oxide-app/src/library/state/methods/apply_primitive_listing.md) |
