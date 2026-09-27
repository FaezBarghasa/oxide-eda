---
okf_version: "0.2"
type: Function
title: urlencoding_minimal
description: "Minimal percent-encoding for the `keyword=` query parameter. Avoids"
resource: crates/oxide-library/src/distributors/lcsc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/lcsc/urlencoding_minimal
language: rust
---

# urlencoding_minimal

Minimal percent-encoding for the `keyword=` query parameter. Avoids

## Signature

```rust
fn urlencoding_minimal(s: &str) -> String
```

## Docstring

Minimal percent-encoding for the `keyword=` query parameter. Avoids
pulling `percent-encoding` as a hard dep — `url::Url` doesn't easily
support partial encoding without a base.

## Source
Lines 212–224 in `crates/oxide-library/src/distributors/lcsc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lcsc](/crates/oxide-library/src/distributors/lcsc.md) |
