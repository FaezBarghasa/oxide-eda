---
okf_version: "0.2"
type: Class
title: FootprintFileWire
description: "On-disk wire shape. Mirrors [`FootprintFile`] but each"
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/FootprintFileWire
language: rust
---

# FootprintFileWire

On-disk wire shape. Mirrors [`FootprintFile`] but each

## Signature

```rust
struct FootprintFileWire
```

## Decorators

- `derive(Serialize, Deserialize)`

## Docstring

On-disk wire shape. Mirrors [`FootprintFile`] but each
[`Footprint`]'s `pads` Vec is replaced with a `pads_tsv: String`
carrying the TSV-encoded payload.
[derive(Serialize, Deserialize)]

## Methods

- `format`
- `file_uuid`
- `display_name`
- `created`
- `updated`
- `footprints`

## Source
Lines 525–534 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
