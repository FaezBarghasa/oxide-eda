---
okf_version: "0.2"
type: Function
title: skip_field_value
description: "Skip a field value: either a single word/number or a `{ ... }` / `[ ... ]` block."
resource: crates/oxide-3d-model-importer/src/vrml/parser/descent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/descent/skip_field_value
language: rust
---

# skip_field_value

Skip a field value: either a single word/number or a `{ ... }` / `[ ... ]` block.

## Signature

```rust
impl Parser<'a> { fn skip_field_value(&mut self) -> Result<(), ParseError> }
```

## Type Parameters

- `'a`

## Docstring

Skip a field value: either a single word/number or a `{ ... }` / `[ ... ]` block.

## Source
Lines 466–475 in `crates/oxide-3d-model-importer/src/vrml/parser/descent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [descent](/crates/oxide-3d-model-importer/src/vrml/parser/descent.md) |
