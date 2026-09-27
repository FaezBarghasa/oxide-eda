---
okf_version: "0.2"
type: Function
title: synthesize_footprint
description: Synthesizes an IPC-compliant Footprint based on package geometry.
resource: crates/oxide-library/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:16:27Z"
concept_id: crates/oxide-library/src/scraper/synthesize_footprint
language: rust
---

# synthesize_footprint

Synthesizes an IPC-compliant Footprint based on package geometry.

## Signature

```rust
pub fn synthesize_footprint(name: &str, pkg: &PackageType) -> Footprint
```

## Visibility

- `pub`

## Docstring

Synthesizes an IPC-compliant Footprint based on package geometry.

## Source
Lines 88–293 in `crates/oxide-library/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/oxide-library/src/scraper.md) |
| called_by | [import_to_library](/crates/oxide-library/src/scraper/import_to_library.md) |
| called_by | [test_synthesize_footprint_chip](/crates/oxide-library/src/scraper/test_synthesize_footprint_chip.md) |
| called_by | [test_synthesize_footprint_soic](/crates/oxide-library/src/scraper/test_synthesize_footprint_soic.md) |
