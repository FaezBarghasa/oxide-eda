---
okf_version: "0.2"
type: Class
title: HarvestedRawData
description: "Raw harvested data payload from distributor APIs, crawlers, or OCR ingestion."
resource: crates/oxide-library/src/harvester/types.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:15Z"
concept_id: crates/oxide-library/src/harvester/types/HarvestedRawData
language: rust
---

# HarvestedRawData

Raw harvested data payload from distributor APIs, crawlers, or OCR ingestion.

## Signature

```rust
pub struct HarvestedRawData
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Raw harvested data payload from distributor APIs, crawlers, or OCR ingestion.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `mpn`
- `manufacturer`
- `description`
- `datasheet_pdf_bytes`
- `datasheet_url`
- `step_model_bytes`
- `raw_spice_model`
- `raw_ibis_model`
- `direct_pins`
- `direct_dimensions`
- `parameters`
- `extra_metadata`

## Source
Lines 254–276 in `crates/oxide-library/src/harvester/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-library/src/harvester/types.md) |
