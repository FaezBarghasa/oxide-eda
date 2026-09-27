---
okf_version: "0.2"
type: Function
title: from_name
description: "Detects package style from package name string (e.g. \"0805\", \"SOIC-8\", \"SOT-23-3\")."
resource: crates/oxide-library/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:16:27Z"
concept_id: crates/oxide-library/src/scraper/from_name_1
language: rust
---

# from_name

Detects package style from package name string (e.g. "0805", "SOIC-8", "SOT-23-3").

## Signature

```rust
pub fn from_name(name: &str) -> Self
```

## Visibility

- `pub`

## Docstring

Detects package style from package name string (e.g. "0805", "SOIC-8", "SOT-23-3").

## Source
Lines 57–79 in `crates/oxide-library/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/oxide-library/src/scraper.md) |
| calls | [extract_trailing_digits](/crates/oxide-library/src/scraper/extract_trailing_digits.md) |
