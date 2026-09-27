---
okf_version: "0.2"
type: Function
title: consume_block
description: "Consume a `{ ... }` or `[ ... ]` block including all nested blocks."
resource: crates/oxide-3d-model-importer/src/vrml/parser/descent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/descent/consume_block_1
language: rust
---

# consume_block

Consume a `{ ... }` or `[ ... ]` block including all nested blocks.

## Signature

```rust
fn consume_block(&mut self) -> Result<(), ParseError>
```

## Docstring

Consume a `{ ... }` or `[ ... ]` block including all nested blocks.

## Source
Lines 478–508 in `crates/oxide-3d-model-importer/src/vrml/parser/descent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [descent](/crates/oxide-3d-model-importer/src/vrml/parser/descent.md) |
