---
okf_version: "0.2"
type: Function
title: display
description: Render the value as a human-readable string. Used by the diff formatter
resource: crates/oxide-library/src/param.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/param/display_1
language: rust
---

# display

Render the value as a human-readable string. Used by the diff formatter

## Signature

```rust
pub fn display(&self) -> String
```

## Visibility

- `pub`

## Docstring

Render the value as a human-readable string. Used by the diff formatter
and the parametric search index.

## Source
Lines 22–29 in `crates/oxide-library/src/param.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [param](/crates/oxide-library/src/param.md) |
