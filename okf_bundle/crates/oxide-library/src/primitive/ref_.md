---
okf_version: "0.2"
type: Module
title: ref_
description: "`PrimitiveRef` — `(library_id, uuid)` address for a reusable primitive."
resource: crates/oxide-library/src/primitive/ref_.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/ref_
language: rust
---

# ref_

`PrimitiveRef` — `(library_id, uuid)` address for a reusable primitive.

## Docstring

`PrimitiveRef` — `(library_id, uuid)` address for a reusable primitive.

Every primitive lives inside a single library and is identified
by its UUID *within that library*. Cross-library references
compose the library UUID (from `library.toml::library_id`) with
the primitive UUID. [`crate::adapters::library_set::LibrarySet`]
resolves these tuples back to the actual primitive struct.

## Relationships

| Type | Target |
|------|--------|
| related | [PrimitiveRef](/crates/oxide-library/src/primitive/ref_/PrimitiveRef.md) |
| related | [new](/crates/oxide-library/src/primitive/ref_/new.md) |
| related | [new](/crates/oxide-library/src/primitive/ref_/new.md) |
| related | [fmt](/crates/oxide-library/src/primitive/ref_/fmt.md) |
| related | [fmt](/crates/oxide-library/src/primitive/ref_/fmt.md) |
| related | [primitive_ref_round_trip](/crates/oxide-library/src/primitive/ref_/primitive_ref_round_trip.md) |
| related | [primitive_ref_display_is_slash_separated](/crates/oxide-library/src/primitive/ref_/primitive_ref_display_is_slash_separated.md) |
| related | [primitive_ref_is_hashable](/crates/oxide-library/src/primitive/ref_/primitive_ref_is_hashable.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
