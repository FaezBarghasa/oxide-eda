---
okf_version: "0.2"
type: Function
title: skip_vrml_header
description: "Skip the `#VRML V2.0 utf8` header line (already tokenized as words)."
resource: crates/oxide-3d-model-importer/src/vrml/parser/descent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/descent/skip_vrml_header_1
language: rust
---

# skip_vrml_header

Skip the `#VRML V2.0 utf8` header line (already tokenized as words).

## Signature

```rust
pub(super) fn skip_vrml_header(&mut self)
```

## Visibility

- `pub(super)`

## Docstring

Skip the `#VRML V2.0 utf8` header line (already tokenized as words).

## Source
Lines 47–51 in `crates/oxide-3d-model-importer/src/vrml/parser/descent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [descent](/crates/oxide-3d-model-importer/src/vrml/parser/descent.md) |
