---
okf_version: "0.2"
type: Class
title: OpenLibrary
description: "One open `*.snxlib/` directory — display cache only. The owning"
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/OpenLibrary
language: rust
---

# OpenLibrary

One open `*.snxlib/` directory — display cache only. The owning

## Signature

```rust
pub struct OpenLibrary
```

## Visibility

- `pub`

## Docstring

One open `*.snxlib/` directory — display cache only. The owning
`LibraryAdapter` lives on [`LibraryState::set`] keyed by
`library_id`.

Components are rows in per-category TSV tables, not standalone
files. The display cache is keyed by table name; each entry holds
the full row payload so the panel can render a grid view per
category without re-reading disk between view ticks. Every row
write triggers a full table reload (v0.9 keeps it simple — hot
per-row patches are a polish item).

## Methods

- `root`
- `display_name`
- `library_id`
- `tables`
- `cached_components`
- `cached_symbols`
- `cached_footprints`
- `cached_sims`
- `display`

## Source
Lines 362–400 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
