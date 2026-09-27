---
okf_version: "0.2"
type: Class
title: LibraryNodeInfo
description: "Per-library bundle for the project tree's `Libraries` group."
resource: crates/oxide-app/src/panels/projects.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/projects/LibraryNodeInfo
language: rust
---

# LibraryNodeInfo

Per-library bundle for the project tree's `Libraries` group.

## Signature

```rust
pub struct LibraryNodeInfo
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Per-library bundle for the project tree's `Libraries` group.
Mirrors what [`oxide_types::project::LibraryEntry`] records on
the project, plus a couple of cached fields the panel pulls from
`LibraryState` so the view doesn't have to re-borrow the library
crate at render time.

The library renders as a single leaf in the project tree under
the v0.9 `.snxlib`-as-file model — symbols / footprints / sims
are not surfaced here. Browsing the library's contents is the
Library Browser tab's job; double-clicking the leaf opens it.
[derive(Debug, Clone)]

## Methods

- `display_name`
- `root`
- `mounted`
- `missing`
- `symbols`
- `footprints`
- `is_open`
- `is_dirty`

## Source
Lines 87–122 in `crates/oxide-app/src/panels/projects.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [projects](/crates/oxide-app/src/panels/projects.md) |
