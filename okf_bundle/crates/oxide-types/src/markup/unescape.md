---
okf_version: "0.2"
type: Function
title: unescape
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/unescape
language: rust
---

# unescape

## Signature

```rust
fn unescape(input: &str) -> String
```

## Source
Lines 433–451 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| called_by | [read_overbar](/crates/oxide-types/src/markup/read_overbar.md) |
| called_by | [read_paired_bracket](/crates/oxide-types/src/markup/read_paired_bracket.md) |
| called_by | [read_paired_double](/crates/oxide-types/src/markup/read_paired_double.md) |
| called_by | [read_paired_paren](/crates/oxide-types/src/markup/read_paired_paren.md) |
| called_by | [read_paired_single](/crates/oxide-types/src/markup/read_paired_single.md) |
