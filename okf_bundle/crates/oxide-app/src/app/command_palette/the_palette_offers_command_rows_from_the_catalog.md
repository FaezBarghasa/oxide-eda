---
okf_version: "0.2"
type: Function
title: the_palette_offers_command_rows_from_the_catalog
description: "#366 — the palette must actually be reading the catalog. A build"
resource: crates/oxide-app/src/app/command_palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command_palette/the_palette_offers_command_rows_from_the_catalog
language: rust
---

# the_palette_offers_command_rows_from_the_catalog

#366 — the palette must actually be reading the catalog. A build

## Signature

```rust
fn the_palette_offers_command_rows_from_the_catalog()
```

## Decorators

- `test`

## Docstring

#366 — the palette must actually be reading the catalog. A build
that produced zero command rows would pass the guard above
vacuously.
[test]

## Source
Lines 383–394 in `crates/oxide-app/src/app/command_palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_palette](/crates/oxide-app/src/app/command_palette.md) |
| calls | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
