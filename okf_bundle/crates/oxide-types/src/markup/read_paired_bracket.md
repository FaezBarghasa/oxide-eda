---
okf_version: "0.2"
type: Function
title: read_paired_bracket
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/read_paired_bracket
language: rust
---

# read_paired_bracket

## Signature

```rust
fn read_paired_bracket(input: &str, start: usize) -> Option<(String, usize)>
```

## Source
Lines 391–410 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| calls | [unescape](/crates/oxide-types/src/markup/unescape.md) |
| called_by | [parse_oxide_markup](/crates/oxide-types/src/markup/parse_oxide_markup.md) |
