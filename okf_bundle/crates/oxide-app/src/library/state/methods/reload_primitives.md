---
okf_version: "0.2"
type: Function
title: reload_primitives
description: "Refresh the cached `(symbols, footprints, sims)` summary"
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/reload_primitives
language: rust
---

# reload_primitives

Refresh the cached `(symbols, footprints, sims)` summary

## Signature

```rust
impl OpenLibrary { pub fn reload_primitives(&mut self, adapter: &dyn LibraryAdapter) }
```

## Visibility

- `pub`

## Docstring

Refresh the cached `(symbols, footprints, sims)` summary
lists. Called by `reload_tables` (full open/refresh path) and
by `save_primitive_tab_at` after a standalone editor write so
the picker modal sees the new primitive without re-scanning
the filesystem on every view tick.

An adapter error keeps the previous cache and emits a tracing
warn — the picker keeps showing the entries it showed a moment
ago until the next successful refresh. Assigning an empty vec
here (what this did before) reported a transient listing
failure as an empty library.

## Source
Lines 508–525 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
| calls | [apply_primitive_listing](/crates/oxide-app/src/library/state/methods/apply_primitive_listing.md) |
