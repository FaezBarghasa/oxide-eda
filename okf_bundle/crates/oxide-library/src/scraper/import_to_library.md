---
okf_version: "0.2"
type: Function
title: import_to_library
description: "Imports a scraped component, its synthesized symbol, footprint, and downloaded datasheet"
resource: crates/oxide-library/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:16:27Z"
concept_id: crates/oxide-library/src/scraper/import_to_library
language: rust
---

# import_to_library

Imports a scraped component, its synthesized symbol, footprint, and downloaded datasheet

## Signature

```rust
impl ComponentScraper { pub fn import_to_library(
        &self,
        adapter: &LocalGitAdapter,
        scraped: &ScrapedComponent,
        table_name: &str,
        pdf_bytes: Option<&[u8]>,
    ) -> Result<ComponentRow, LibraryError> }
```

## Visibility

- `pub`

## Docstring

Imports a scraped component, its synthesized symbol, footprint, and downloaded datasheet
directly into a local `.snxlib` library.
[cfg(feature = "local-git")]

## Source
Lines 574–682 in `crates/oxide-library/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/oxide-library/src/scraper.md) |
| calls | [synthesize_footprint](/crates/oxide-library/src/scraper/synthesize_footprint.md) |
| calls | [synthesize_symbol](/crates/oxide-library/src/scraper/synthesize_symbol.md) |
| calls | [library_id](/crates/oxide-library/src/adapter/library_id.md) |
| calls | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |
