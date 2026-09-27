---
okf_version: "0.2"
type: Class
title: IconId
description: "Surface-agnostic icon key. A view surface (menu, toolbar, command"
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/IconId
language: rust
---

# IconId

Surface-agnostic icon key. A view surface (menu, toolbar, command

## Signature

```rust
pub struct IconId
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Surface-agnostic icon key. A view surface (menu, toolbar, command
palette) maps the key to its actual glyph/asset; the catalog stays a
plain identifier so adding an icon never means adding a variant here.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 51–51 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
