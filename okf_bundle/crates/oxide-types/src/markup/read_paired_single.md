---
okf_version: "0.2"
type: Function
title: read_paired_single
description: Find the next single sigil byte and capture the content between.
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/read_paired_single
language: rust
---

# read_paired_single

Find the next single sigil byte and capture the content between.

## Signature

```rust
fn read_paired_single(input: &str, start: usize, sigil: u8) -> Option<(String, usize)>
```

## Docstring

Find the next single sigil byte and capture the content between.

## Source
Lines 326–345 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| calls | [unescape](/crates/oxide-types/src/markup/unescape.md) |
| called_by | [parse_oxide_markup](/crates/oxide-types/src/markup/parse_oxide_markup.md) |
