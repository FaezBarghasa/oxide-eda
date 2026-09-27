---
okf_version: "0.2"
type: Function
title: probe_every_kind
description: "Try `r` as a symbol, then a footprint, then a sim model, stopping at"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/probe_every_kind
language: rust
---

# probe_every_kind

Try `r` as a symbol, then a footprint, then a sim model, stopping at

## Signature

```rust
fn probe_every_kind(lib: &dyn LibraryAdapter, r: &PrimitiveRef) -> Probe
```

## Docstring

Try `r` as a symbol, then a footprint, then a sim model, stopping at
the first hit. Only an all-`NotFound` sweep proves the reference is
missing.

## Source
Lines 304–321 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| called_by | [unresolved_refs](/crates/oxide-library/src/adapters/library_set/unresolved_refs.md) |
