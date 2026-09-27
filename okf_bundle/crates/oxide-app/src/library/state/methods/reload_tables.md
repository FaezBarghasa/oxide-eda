---
okf_version: "0.2"
type: Function
title: reload_tables
description: Re-read every TSV via the supplied adapter. Replaces both
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/reload_tables
language: rust
---

# reload_tables

Re-read every TSV via the supplied adapter. Replaces both

## Signature

```rust
impl OpenLibrary { pub fn reload_tables(&mut self, adapter: &dyn LibraryAdapter) -> Result<(), LibraryError> }
```

## Visibility

- `pub`

## Docstring

Re-read every TSV via the supplied adapter. Replaces both
`tables` and `cached_components` atomically — readers see one
consistent snapshot regardless of which view they query.

`LibraryError::Backend` from the adapter (e.g. an adapter that
hasn't implemented `list_tables` yet) is propagated to the
caller; partial table loads on per-table read errors are
tolerated and merely warn through `tracing`.

## Source
Lines 462–495 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
