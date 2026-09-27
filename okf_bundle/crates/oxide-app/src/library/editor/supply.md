---
okf_version: "0.2"
type: Module
title: supply
description: Supply tab — primary MPN + ranked alternates + distributor listings
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply
language: rust
---

# supply

Supply tab — primary MPN + ranked alternates + distributor listings

## Docstring

Supply tab — primary MPN + ranked alternates + distributor listings
editor. Retargeted to `ComponentRow` (DBLib model) per
`v0.9-refactor-2-plan.md` §11.4.

The shape this view edits:

- `state.row.primary_mpn`        : `ManufacturerPart` — headline part.
- `state.row.alternates`         : `Vec<ManufacturerPart>` — AVL.
- `state.row.supply`             : `Vec<DistributorListing>` — sourcing rows.

Layout is three muted-header sections:

1. **Primary MPN** — 4-row form (Manufacturer / MPN / Status / Notes).
2. **Alternates** — one inline row per alternate (manufacturer, MPN,
status pick_list, notes, remove [×]) plus a `+ Add Alternate`
trigger.
3. **Distributor Listings** — table-like rows (distributor pick_list,
sku, url, remove [×]) plus a `+ Add Listing` trigger.

Every value mutation flows through `EditorMsg::Supply*` → the
library dispatcher's `apply_inline_edit` arms (see `dispatch/library.rs`),
which write directly to `editor.draft.*` and bump `editor.dirty`.

## Relationships

| Type | Target |
|------|--------|
| related | [StatusPick](/crates/oxide-app/src/library/editor/supply/StatusPick.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/supply/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/supply/fmt.md) |
| related | [DistributorPick](/crates/oxide-app/src/library/editor/supply/DistributorPick.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/supply/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/supply/fmt.md) |
| related | [distributor_label](/crates/oxide-app/src/library/editor/supply/distributor_label.md) |
| related | [distributor_from_label](/crates/oxide-app/src/library/editor/supply/distributor_from_label.md) |
| related | [distributor_source_to_string](/crates/oxide-app/src/library/editor/supply/distributor_source_to_string.md) |
| related | [view](/crates/oxide-app/src/library/editor/supply/view.md) |
| related | [section_header](/crates/oxide-app/src/library/editor/supply/section_header.md) |
| related | [primary_form](/crates/oxide-app/src/library/editor/supply/primary_form.md) |
| related | [alternates_section](/crates/oxide-app/src/library/editor/supply/alternates_section.md) |
| related | [alternate_row](/crates/oxide-app/src/library/editor/supply/alternate_row.md) |
| related | [listings_section](/crates/oxide-app/src/library/editor/supply/listings_section.md) |
| related | [listing_row](/crates/oxide-app/src/library/editor/supply/listing_row.md) |
| related | [labelled_input](/crates/oxide-app/src/library/editor/supply/labelled_input.md) |
| related | [add_button](/crates/oxide-app/src/library/editor/supply/add_button.md) |
| related | [remove_button](/crates/oxide-app/src/library/editor/supply/remove_button.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
