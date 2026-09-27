---
okf_version: "0.2"
type: Function
title: parse
resource: crates/oxide-library/src/manifest.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/manifest/parse
language: rust
---

# parse

## Signature

```rust
impl Manifest { pub fn parse(text: &str) -> Result<Self, toml::de::Error> }
```

## Visibility

- `pub`

## Source
Lines 141–143 in `crates/oxide-library/src/manifest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manifest](/crates/oxide-library/src/manifest.md) |
