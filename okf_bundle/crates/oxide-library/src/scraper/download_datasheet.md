---
okf_version: "0.2"
type: Function
title: download_datasheet
description: Downloads a datasheet PDF over HTTP with stealth headers and calculates its SHA-256 hash.
resource: crates/oxide-library/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:16:27Z"
concept_id: crates/oxide-library/src/scraper/download_datasheet
language: rust
---

# download_datasheet

Downloads a datasheet PDF over HTTP with stealth headers and calculates its SHA-256 hash.

## Signature

```rust
impl ComponentScraper { pub fn download_datasheet(&self, url: &Url) -> Result<(Vec<u8>, String), String> }
```

## Visibility

- `pub`

## Docstring

Downloads a datasheet PDF over HTTP with stealth headers and calculates its SHA-256 hash.

## Source
Lines 522–555 in `crates/oxide-library/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/oxide-library/src/scraper.md) |
