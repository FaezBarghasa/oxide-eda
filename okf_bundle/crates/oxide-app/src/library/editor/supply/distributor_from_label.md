---
okf_version: "0.2"
type: Function
title: distributor_from_label
description: "Best-effort reverse of `distributor_label` — turn the canonical string"
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/distributor_from_label
language: rust
---

# distributor_from_label

Best-effort reverse of `distributor_label` — turn the canonical string

## Signature

```rust
fn distributor_from_label(s: &str) -> DistributorSource
```

## Docstring

Best-effort reverse of `distributor_label` — turn the canonical string
stored on `DistributorListing.distributor` back into a
`DistributorSource` for the pick_list selection. Unknown / legacy
strings select `Other` so the picker can still drive the row.

## Source
Lines 99–109 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| called_by | [listing_row](/crates/oxide-app/src/library/editor/supply/listing_row.md) |
