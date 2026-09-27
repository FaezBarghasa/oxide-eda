---
okf_version: "0.2"
type: Function
title: source_to_str
description: Wire-name for a distributor source — kept in this module so the
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/source_to_str
language: rust
---

# source_to_str

Wire-name for a distributor source — kept in this module so the

## Signature

```rust
fn source_to_str(s: DistributorSource) -> &'static str
```

## Docstring

Wire-name for a distributor source — kept in this module so the
schema is local to the file that owns it.

## Source
Lines 73–83 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| called_by | [source_str_round_trip_covers_every_variant](/crates/oxide-app/src/library/settings/persistence/source_str_round_trip_covers_every_variant.md) |
