---
okf_version: "0.2"
type: Class
title: SnxSchematic
description: "On-disk representation of a `.snxsch` file."
resource: crates/oxide-types/src/format/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/format/mod/SnxSchematic
language: rust
---

# SnxSchematic

On-disk representation of a `.snxsch` file.

## Signature

```rust
pub struct SnxSchematic
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

On-disk representation of a `.snxsch` file.

Internally constructed from a [`SchematicSheet`] via
[`SnxSchematic::new`] (which decomposes the sheet into bulk TSV
rows + an extras-TOML auxiliary table that captures every field
the row schema doesn't cover) and rebuilt via [`SnxSchematic::parse`]
(which round-trips back to a fully-populated [`SchematicSheet`]).

Callers that just want the in-memory sheet read `self.sheet`.
[derive(Debug, Clone)]

## Methods

- `format`
- `sheet`

## Source
Lines 147–153 in `crates/oxide-types/src/format/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [format](/crates/oxide-types/src/format/mod.md) |
