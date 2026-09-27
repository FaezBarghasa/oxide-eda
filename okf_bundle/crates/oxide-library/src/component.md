---
okf_version: "0.2"
type: Module
title: component
description: "`ComponentRow` — one row of a component table (Altium DBLib model)."
resource: crates/oxide-library/src/component.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/component
language: rust
---

# component

`ComponentRow` — one row of a component table (Altium DBLib model).

## Docstring

`ComponentRow` — one row of a component table (Altium DBLib model).

Per `v0.9-refactor-2-plan.md` §2.1, a "component" is no longer a file
holding a chain of revisions. It's a single row inside a category table
(`tables/<name>.tsv` for LocalGit; one record in `component_rows` for the
database backend). Symbols, footprints, and sim models stay as standalone
editable primitive files referenced by `(library_id, uuid)` tuples.

Two MPNs sharing a SOIC-8 footprint reference the same primitive UUID
rather than carrying their own copy.

## Relationships

| Type | Target |
|------|--------|
| related | [DatasheetRef](/crates/oxide-library/src/component/DatasheetRef.md) |
| related | [url](/crates/oxide-library/src/component/url.md) |
| related | [hash_pinned](/crates/oxide-library/src/component/hash_pinned.md) |
| related | [url](/crates/oxide-library/src/component/url.md) |
| related | [hash_pinned](/crates/oxide-library/src/component/hash_pinned.md) |
| related | [default](/crates/oxide-library/src/component/default.md) |
| related | [default](/crates/oxide-library/src/component/default.md) |
| related | [default_row_version](/crates/oxide-library/src/component/default_row_version.md) |
| related | [PinPadOverride](/crates/oxide-library/src/component/PinPadOverride.md) |
| related | [new](/crates/oxide-library/src/component/new.md) |
| related | [new](/crates/oxide-library/src/component/new.md) |
| related | [PlmReserved](/crates/oxide-library/src/component/PlmReserved.md) |
| related | [ComponentRow](/crates/oxide-library/src/component/ComponentRow.md) |
| related | [refresh_content_hash](/crates/oxide-library/src/component/refresh_content_hash.md) |
| related | [refresh_content_hash](/crates/oxide-library/src/component/refresh_content_hash.md) |
| related | [fixture_row](/crates/oxide-library/src/component/fixture_row.md) |
| related | [component_row_json_roundtrip](/crates/oxide-library/src/component/component_row_json_roundtrip.md) |
| related | [refresh_content_hash_populates](/crates/oxide-library/src/component/refresh_content_hash_populates.md) |
| related | [datasheet_ref_round_trip_each_variant](/crates/oxide-library/src/component/datasheet_ref_round_trip_each_variant.md) |
| related | [pin_pad_override_round_trip](/crates/oxide-library/src/component/pin_pad_override_round_trip.md) |
| related | [plm_reserved_defaults_round_trip_clean](/crates/oxide-library/src/component/plm_reserved_defaults_round_trip_clean.md) |
| related | [row_id_wraps_bare_uuid_field](/crates/oxide-library/src/component/row_id_wraps_bare_uuid_field.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
