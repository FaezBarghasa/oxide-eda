---
okf_version: "0.2"
type: Module
title: library_place
description: Place-from-library flow handler.
resource: crates/oxide-app/src/app/handlers/library_place.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/library_place
language: rust
---

# library_place

Place-from-library flow handler.

## Docstring

Place-from-library flow handler.

v0.9-refactor-2 (DBLib model): the place flow is keyed by
`(library_path, table, row_id)`. Resolution path:

1. Look up the open library by path → grab its `library_id`.
2. Read the picked row through the mounted adapter
(`LibrarySet::get(library_id)`).
3. Resolve the row's `symbol_ref` via `LibrarySet::resolve_symbol`
so the trace records the embedded-symbol presence and pin
count alongside the row's content hash.
4. Trace the structured place-flow fields and close the picker.

The actual schematic-engine embed (writing the
`LibrarySourceRef { library_id, uuid, version, content_hash }` +
embedded slice + shared snapshot onto the placed
`oxide_types::schematic::Symbol`) lands in v0.9 Phase 3 once the
schematic-side schema slots exist. Until then this handler
exercises the dispatch routing end-to-end and gives operators a
correlatable trace per place gesture.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_place_library_component](/crates/oxide-app/src/app/handlers/library_place/handle_place_library_component.md) |
| related | [handle_place_library_component](/crates/oxide-app/src/app/handlers/library_place/handle_place_library_component.md) |
| related | [hex_short](/crates/oxide-app/src/app/handlers/library_place/hex_short.md) |
| related | [place_message_from_picker](/crates/oxide-app/src/app/handlers/library_place/place_message_from_picker.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
