---
okf_version: "0.2"
type: Function
title: read_paired_double
description: "Find a doubled sigil (e.g. `**` or `~~`) and capture the content between."
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/read_paired_double
language: rust
---

# read_paired_double

Find a doubled sigil (e.g. `**` or `~~`) and capture the content between.

## Signature

```rust
fn read_paired_double(input: &str, start: usize, sigil: u8) -> Option<(String, usize)>
```

## Docstring

Find a doubled sigil (e.g. `**` or `~~`) and capture the content between.

## Source
Lines 348–367 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| calls | [unescape](/crates/oxide-types/src/markup/unescape.md) |
| called_by | [parse_oxide_markup](/crates/oxide-types/src/markup/parse_oxide_markup.md) |
