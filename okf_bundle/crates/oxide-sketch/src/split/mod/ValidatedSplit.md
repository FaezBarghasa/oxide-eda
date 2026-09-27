---
okf_version: "0.2"
type: Class
title: ValidatedSplit
description: "Read-only results of [`validate_split`]'s checks — the retired"
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/ValidatedSplit
language: rust
---

# ValidatedSplit

Read-only results of [`validate_split`]'s checks — the retired

## Signature

```rust
struct ValidatedSplit
```

## Docstring

Read-only results of [`validate_split`]'s checks — the retired
line's index plus its resolved endpoint ids/coordinates. Named
fields instead of a 5-tuple `Result` payload (clippy's
`type_complexity` flags a bare tuple that size).

## Methods

- `line_idx`
- `start_id`
- `end_id`
- `start_xy`
- `end_xy`

## Source
Lines 278–284 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
