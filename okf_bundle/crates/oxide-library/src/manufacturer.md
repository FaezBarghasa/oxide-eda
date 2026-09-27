---
okf_version: "0.2"
type: Module
title: manufacturer
description: Manufacturer-part + supply-chain types.
resource: crates/oxide-library/src/manufacturer.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/manufacturer
language: rust
---

# manufacturer

Manufacturer-part + supply-chain types.

## Docstring

Manufacturer-part + supply-chain types.

Per `v0.9-refactor-2-plan.md` §2.4, every `Revision` carries a
`primary_mpn: ManufacturerPart`, a ranked list of `alternates`, and a list
of `DistributorListing` entries describing where the part can be sourced.

These types intentionally live in a separate module from the legacy
`distributor` adapters: an MPN/AVL entry is a *static* property of a
component revision, while distributor-cache and live-pricing belong to the
`distributor.rs` runtime adapters.

## Relationships

| Type | Target |
|------|--------|
| related | [AlternateStatus](/crates/oxide-library/src/manufacturer/AlternateStatus.md) |
| related | [ManufacturerPart](/crates/oxide-library/src/manufacturer/ManufacturerPart.md) |
| related | [draft](/crates/oxide-library/src/manufacturer/draft.md) |
| related | [draft](/crates/oxide-library/src/manufacturer/draft.md) |
| related | [DistributorListing](/crates/oxide-library/src/manufacturer/DistributorListing.md) |
| related | [new](/crates/oxide-library/src/manufacturer/new.md) |
| related | [new](/crates/oxide-library/src/manufacturer/new.md) |
| related | [manufacturer_part_round_trip](/crates/oxide-library/src/manufacturer/manufacturer_part_round_trip.md) |
| related | [alternate_status_round_trip_all_variants](/crates/oxide-library/src/manufacturer/alternate_status_round_trip_all_variants.md) |
| related | [distributor_listing_round_trip](/crates/oxide-library/src/manufacturer/distributor_listing_round_trip.md) |
| related | [manufacturer_part_draft_is_primary](/crates/oxide-library/src/manufacturer/manufacturer_part_draft_is_primary.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
