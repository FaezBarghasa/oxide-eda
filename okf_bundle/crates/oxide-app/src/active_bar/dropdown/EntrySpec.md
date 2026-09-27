---
okf_version: "0.2"
type: Class
title: EntrySpec
description: One row inside a uniform per-menu entry table — theme- and
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/EntrySpec
language: rust
---

# EntrySpec

One row inside a uniform per-menu entry table — theme- and

## Signature

```rust
enum EntrySpec
```

## Docstring

One row inside a uniform per-menu entry table — theme- and
context-free, unlike `DropdownEntry`/`DropdownItem`, so the table
itself can be a `const` array (icon fn pointers and fieldless enum
variants are const-constructible; `ActiveBarAction` is not `Copy`,
so `render` clones it per row). Collapses the ~78 near-identical
`dd_item(...)` call sites this file used to carry across ~11
uniform per-menu builder functions into ~11 data tables + one
renderer (#457, epic #278).

## Methods

- `icon`
- `label`
- `action`

## Source
Lines 117–124 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
