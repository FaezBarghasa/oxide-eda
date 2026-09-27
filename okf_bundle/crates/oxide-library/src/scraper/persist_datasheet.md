---
okf_version: "0.2"
type: Function
title: persist_datasheet
description: "Saves the downloaded datasheet bytes into the library's `datasheets/` directory."
resource: crates/oxide-library/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:16:27Z"
concept_id: crates/oxide-library/src/scraper/persist_datasheet
language: rust
---

# persist_datasheet

Saves the downloaded datasheet bytes into the library's `datasheets/` directory.

## Signature

```rust
impl ComponentScraper { pub fn persist_datasheet(
        &self,
        library_root: &Path,
        filename: &str,
        bytes: &[u8],
    ) -> Result<PathBuf, std::io::Error> }
```

## Visibility

- `pub`

## Docstring

Saves the downloaded datasheet bytes into the library's `datasheets/` directory.

## Source
Lines 558–569 in `crates/oxide-library/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/oxide-library/src/scraper.md) |
