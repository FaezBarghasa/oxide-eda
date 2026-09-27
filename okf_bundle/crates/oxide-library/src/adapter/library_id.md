---
okf_version: "0.2"
type: Function
title: library_id
description: "Stable UUID of this library, sourced from `library.toml::library.library_id`."
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/library_id
language: rust
---

# library_id

Stable UUID of this library, sourced from `library.toml::library.library_id`.

## Signature

```rust
fn library_id(&self) -> Uuid
```

## Docstring

Stable UUID of this library, sourced from `library.toml::library.library_id`.

Used by [`crate::adapters::library_set::LibrarySet`] to key resolution
of cross-library [`crate::primitive::PrimitiveRef`]s. The default
implementation pulls it from `manifest().library.library_id` so any
adapter whose manifest is honest gets it for free.

## Source
Lines 149–151 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
| called_by | [blank_open_library](/crates/oxide-app/tests/measure_library_open/blank_open_library.md) |
| called_by | [library_with_a_measurement_row](/crates/oxide-app/tests/regression/library_browser_cell_commit/library_with_a_measurement_row.md) |
| called_by | [append_component](/crates/oxide-app/tests/support/mod/append_component.md) |
| called_by | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
| called_by | [find_adapter_for_id](/crates/oxide-library/src/adapters/library_set/find_adapter_for_id.md) |
| called_by | [find_key_for_id](/crates/oxide-library/src/adapters/library_set/find_key_for_id.md) |
| called_by | [for_adapter](/crates/oxide-library/src/adapters/library_set/for_adapter.md) |
| called_by | [library_ids](/crates/oxide-library/src/adapters/library_set/library_ids.md) |
| called_by | [import_to_library](/crates/oxide-library/src/scraper/import_to_library.md) |
