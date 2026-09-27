---
okf_version: "0.2"
type: Function
title: new
description: "Creates a stealth scraper client configured with TLS fingerprinting hygiene,"
resource: crates/oxide-library/src/scraper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:16:27Z"
concept_id: crates/oxide-library/src/scraper/new
language: rust
---

# new

Creates a stealth scraper client configured with TLS fingerprinting hygiene,

## Signature

```rust
impl ComponentScraper { pub fn new() -> Self }
```

## Visibility

- `pub`

## Docstring

Creates a stealth scraper client configured with TLS fingerprinting hygiene,
standard browser HTTP headers (Sec-CH-UA, Sec-Fetch-*, Accept, etc.) to evade
bot, AI-crawler, and WAF fingerprinting.

## Source
Lines 404–440 in `crates/oxide-library/src/scraper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scraper](/crates/oxide-library/src/scraper.md) |
