---
okf_version: "0.2"
type: Function
title: from_hex
description: "Parse \"#RRGGBB\" or \"#RRGGBBAA\" hex string."
resource: crates/oxide-types/src/theme.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/theme/from_hex
language: rust
---

# from_hex

Parse "#RRGGBB" or "#RRGGBBAA" hex string.

## Signature

```rust
impl Color { pub fn from_hex(hex: &str) -> Self }
```

## Visibility

- `pub`

## Docstring

Parse "#RRGGBB" or "#RRGGBBAA" hex string.

## Source
Lines 46–57 in `crates/oxide-types/src/theme.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme](/crates/oxide-types/src/theme.md) |
