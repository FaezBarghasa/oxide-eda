---
okf_version: "0.2"
type: Function
title: editor_for
description: "Existing editor for `(library_root, table, row_id)`, if any."
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/editor_for
language: rust
---

# editor_for

Existing editor for `(library_root, table, row_id)`, if any.

## Signature

```rust
impl LibraryState { pub fn editor_for(
        &self,
        library_root: &Path,
        table: &str,
        row_id: RowId,
    ) -> Option<&ComponentPreviewState> }
```

## Visibility

- `pub`

## Docstring

Existing editor for `(library_root, table, row_id)`, if any.

## Source
Lines 338–349 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
