---
okf_version: "0.2"
type: Function
title: handle_place_library_component
description: "Resolve a `(library, table, row_id)` selection through the"
resource: crates/oxide-app/src/app/handlers/library_place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/library_place/handle_place_library_component_1
language: rust
---

# handle_place_library_component

Resolve a `(library, table, row_id)` selection through the

## Signature

```rust
pub(crate) fn handle_place_library_component(
        &mut self,
        library_path: PathBuf,
        table: String,
        row_id: RowId,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Resolve a `(library, table, row_id)` selection through the
mounted adapter + `LibrarySet`, then close the picker. Always
returns [`Task::none()`] — Phase 3 will swap the structured
trace for an engine command that writes the embedded fields
onto the placed `Symbol`.

## Source
Lines 37–116 in `crates/oxide-app/src/app/handlers/library_place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_place](/crates/oxide-app/src/app/handlers/library_place.md) |
