---
okf_version: "0.2"
type: Module
title: datasheet_picker
description: Datasheet tab — small pick_list + value control bound to
resource: crates/oxide-app/src/library/editor/datasheet_picker.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/datasheet_picker
language: rust
---

# datasheet_picker

Datasheet tab — small pick_list + value control bound to

## Docstring

Datasheet tab — small pick_list + value control bound to
`state.row.datasheet`. Standalone Component-Preview tab per
`v0.9-refactor-2-plan.md` §11.5.

Two modes:
* **URL** — plain text input. Sets `DatasheetRef::Url`.
* **Pinned PDF** — file picker. Hashes the bytes and sets
`DatasheetRef::HashPinned { hash, filename }`.

The mode is derived from the row's `DatasheetRef` variant.

## Relationships

| Type | Target |
|------|--------|
| related | [DatasheetMode](/crates/oxide-app/src/library/editor/datasheet_picker/DatasheetMode.md) |
| related | [from_ref](/crates/oxide-app/src/library/editor/datasheet_picker/from_ref.md) |
| related | [from_ref](/crates/oxide-app/src/library/editor/datasheet_picker/from_ref.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/datasheet_picker/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/datasheet_picker/fmt.md) |
| related | [view](/crates/oxide-app/src/library/editor/datasheet_picker/view.md) |
| related | [view_url_input](/crates/oxide-app/src/library/editor/datasheet_picker/view_url_input.md) |
| related | [view_pinned_input](/crates/oxide-app/src/library/editor/datasheet_picker/view_pinned_input.md) |
| related | [mode_from_ref_url_variant](/crates/oxide-app/src/library/editor/datasheet_picker/mode_from_ref_url_variant.md) |
| related | [mode_from_ref_hash_pinned_variant](/crates/oxide-app/src/library/editor/datasheet_picker/mode_from_ref_hash_pinned_variant.md) |
| related | [mode_from_ref_none_falls_back_to_url](/crates/oxide-app/src/library/editor/datasheet_picker/mode_from_ref_none_falls_back_to_url.md) |
| related | [mode_display_strings_are_stable](/crates/oxide-app/src/library/editor/datasheet_picker/mode_display_strings_are_stable.md) |
| related | [datasheet_ref_round_trips_via_json_url](/crates/oxide-app/src/library/editor/datasheet_picker/datasheet_ref_round_trips_via_json_url.md) |
| related | [datasheet_ref_round_trips_via_json_hash_pinned](/crates/oxide-app/src/library/editor/datasheet_picker/datasheet_ref_round_trips_via_json_hash_pinned.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
