---
okf_version: "0.2"
type: Function
title: pinned_unmapped_ids_still_exist_in_the_catalog
description: "The pinned ids must still exist in the catalog, so the list cannot"
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/pinned_unmapped_ids_still_exist_in_the_catalog
language: rust
---

# pinned_unmapped_ids_still_exist_in_the_catalog

The pinned ids must still exist in the catalog, so the list cannot

## Signature

```rust
fn pinned_unmapped_ids_still_exist_in_the_catalog()
```

## Decorators

- `test`

## Docstring

The pinned ids must still exist in the catalog, so the list cannot
rot into a set of names that no longer mean anything.
[test]

## Source
Lines 452–464 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [all_command_ids](/crates/oxide-app/src/keymap/catalog/mod/all_command_ids.md) |
