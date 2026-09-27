---
okf_version: "0.2"
type: Function
title: refresh_primitive_cache_for
description: "Refresh the matching library's per-kind primitive cache so the"
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/refresh_primitive_cache_for_1
language: rust
---

# refresh_primitive_cache_for

Refresh the matching library's per-kind primitive cache so the

## Signature

```rust
fn refresh_primitive_cache_for(&mut self, path: &std::path::Path)
```

## Docstring

Refresh the matching library's per-kind primitive cache so the
picker modal sees the just-saved primitive without waiting
for the next full `refresh_components` round-trip. No-op when
`path` lives outside any mounted library.

A failing listing must not blank the caches. This runs right
after a primitive save, so the three caches it touches are
known-good; overwriting them with empty vecs made every symbol,
footprint and sim vanish from the library's picker and read as
"my save didn't work" while the file was fine on disk.
[`OpenLibrary::reload_primitives`] keeps each previous cache on
error and reports the failure to the Messages panel at `warn`.

## Source
Lines 564–590 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
