---
okf_version: "0.2"
type: Function
title: standard_title_block
description: Shared title-block layout used by every built-in. 180 x 40 mm rectangle
resource: crates/oxide-output/src/template/builtin.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/template/builtin/standard_title_block
language: rust
---

# standard_title_block

Shared title-block layout used by every built-in. 180 x 40 mm rectangle

## Signature

```rust
fn standard_title_block() -> TitleBlock
```

## Docstring

Shared title-block layout used by every built-in. 180 x 40 mm rectangle
anchored at the page's bottom-right, with six fields stacked over two
columns. Matches the sketch in OUTPUT_PLAN.md §4.

## Source
Lines 159–220 in `crates/oxide-output/src/template/builtin.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builtin](/crates/oxide-output/src/template/builtin.md) |
| called_by | [load_builtin](/crates/oxide-output/src/template/builtin/load_builtin.md) |
