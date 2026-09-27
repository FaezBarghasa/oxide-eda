---
okf_version: "0.2"
type: Function
title: from_row
description: Build a preview state from a freshly-loaded row.
resource: crates/oxide-app/src/library/state/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/state/preview/from_row_1
language: rust
---

# from_row

Build a preview state from a freshly-loaded row.

## Signature

```rust
pub fn from_row(library_path: PathBuf, table: String, row: ComponentRow) -> Self
```

## Visibility

- `pub`

## Docstring

Build a preview state from a freshly-loaded row.

## Source
Lines 202–216 in `crates/oxide-app/src/library/state/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/state/preview.md) |
