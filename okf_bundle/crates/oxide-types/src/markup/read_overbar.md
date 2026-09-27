---
okf_version: "0.2"
type: Function
title: read_overbar
description: "Read the body of an overbar `_~ ... ~_` (already past the opening `_~`)."
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/read_overbar
language: rust
---

# read_overbar

Read the body of an overbar `_~ ... ~_` (already past the opening `_~`).

## Signature

```rust
fn read_overbar(input: &str, start: usize) -> Option<(String, usize)>
```

## Docstring

Read the body of an overbar `_~ ... ~_` (already past the opening `_~`).

## Source
Lines 370–389 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| calls | [unescape](/crates/oxide-types/src/markup/unescape.md) |
| called_by | [parse_oxide_markup](/crates/oxide-types/src/markup/parse_oxide_markup.md) |
