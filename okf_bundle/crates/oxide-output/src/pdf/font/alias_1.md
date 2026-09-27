---
okf_version: "0.2"
type: Function
title: alias
description: "Short alias used inside content streams (`/F1 9 Tf ...`). Matches the"
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/alias_1
language: rust
---

# alias

Short alias used inside content streams (`/F1 9 Tf ...`). Matches the

## Signature

```rust
pub fn alias(&self) -> &'static str
```

## Visibility

- `pub`

## Docstring

Short alias used inside content streams (`/F1 9 Tf ...`). Matches the
keys emitted in the page's /Font resources dict.

## Source
Lines 62–69 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
