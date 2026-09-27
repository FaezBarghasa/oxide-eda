---
okf_version: "0.2"
type: Function
title: every_catalog_entry_sets_enable_and_flags
description: "Every catalog row must state `enable` and `flags` explicitly."
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/every_catalog_entry_sets_enable_and_flags
language: rust
---

# every_catalog_entry_sets_enable_and_flags

Every catalog row must state `enable` and `flags` explicitly.

## Signature

```rust
fn every_catalog_entry_sets_enable_and_flags()
```

## Decorators

- `test`

## Docstring

Every catalog row must state `enable` and `flags` explicitly.

A source scan rather than a value check, because the two cannot be
told apart at runtime: `Enablement::Always` + `CommandFlags::NONE`
is both a legitimate answer and what an unpopulated row inherits
from `..CommandMetadata::DEFAULT`. Only the source says whether
somebody decided. Mirrors the source-scanning style of
`keymap::menu_command_tests` and `app::command::bridge`'s guards.
[test]

## Source
Lines 459–503 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
