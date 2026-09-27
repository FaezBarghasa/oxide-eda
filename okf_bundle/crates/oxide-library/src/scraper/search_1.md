---
okf_version: "0.2"
type: Function
title: search
description: Searches online component distributors for the given query/MPN with stealth headers and anti-detection.
resource: crates/oxide-library/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:16:27Z"
concept_id: crates/oxide-library/src/scraper/search_1
language: rust
---

# search

Searches online component distributors for the given query/MPN with stealth headers and anti-detection.

## Signature

```rust
pub fn search(&self, query: &str) -> Result<Vec<ScrapedComponent>, String>
```

## Visibility

- `pub`

## Docstring

Searches online component distributors for the given query/MPN with stealth headers and anti-detection.

## Source
Lines 443–519 in `crates/oxide-library/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/oxide-library/src/scraper.md) |
