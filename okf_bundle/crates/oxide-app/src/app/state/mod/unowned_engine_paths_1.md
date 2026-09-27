---
okf_version: "0.2"
type: Function
title: unowned_engine_paths
description: Open engine paths that belong to no loaded project — the page set of a
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/unowned_engine_paths_1
language: rust
---

# unowned_engine_paths

Open engine paths that belong to no loaded project — the page set of a

## Signature

```rust
pub fn unowned_engine_paths(&self) -> Vec<PathBuf>
```

## Visibility

- `pub`

## Docstring

Open engine paths that belong to no loaded project — the page set of a
loose-document export. An open tab belonging to some *other* loaded
project would otherwise ride along as an extra page.

Sorted by path: `engines` is a `HashMap`, so an unsorted result orders
the loose export's pages (and the print-preview file-picker rows, which
are re-seeded from this set on every rerasterize) by hash iteration
order — visibly reshuffling between rerasterizes with two or more loose
schematics open. Callers that want a specific page first re-order after.

## Source
Lines 690–700 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
| calls | [project_owning_sheet](/crates/oxide-app/src/app/state/scope/project_owning_sheet.md) |
