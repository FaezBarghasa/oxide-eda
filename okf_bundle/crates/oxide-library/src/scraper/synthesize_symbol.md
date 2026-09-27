---
okf_version: "0.2"
type: Function
title: synthesize_symbol
description: Synthesizes a schematic Symbol with bounding box graphics and pins.
resource: crates/oxide-library/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:16:27Z"
concept_id: crates/oxide-library/src/scraper/synthesize_symbol
language: rust
---

# synthesize_symbol

Synthesizes a schematic Symbol with bounding box graphics and pins.

## Signature

```rust
pub fn synthesize_symbol(
    name: &str,
    pins_data: &[(String, String, PinDirection)],
    designator_prefix: &str,
) -> Symbol
```

## Visibility

- `pub`

## Docstring

Synthesizes a schematic Symbol with bounding box graphics and pins.

## Source
Lines 296–377 in `crates/oxide-library/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/oxide-library/src/scraper.md) |
| called_by | [import_to_library](/crates/oxide-library/src/scraper/import_to_library.md) |
| called_by | [test_synthesize_symbol](/crates/oxide-library/src/scraper/test_synthesize_symbol.md) |
