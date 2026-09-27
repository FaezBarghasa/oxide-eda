---
okf_version: "0.2"
type: Function
title: every_bridged_command_id_resolves_in_the_catalog
description: "Drift guard: every id `core_to_message` matches must resolve in"
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/every_bridged_command_id_resolves_in_the_catalog
language: rust
---

# every_bridged_command_id_resolves_in_the_catalog

Drift guard: every id `core_to_message` matches must resolve in

## Signature

```rust
fn every_bridged_command_id_resolves_in_the_catalog()
```

## Decorators

- `test`

## Docstring

Drift guard: every id `core_to_message` matches must resolve in
the command catalog via [`metadata_for`]. `menu_command_tests`
guards the menu *views*; this is the analogous guard for the
bridge itself, which nothing scanned before this slice.
[test]

## Source
Lines 274–298 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [bridged_command_ids](/crates/oxide-app/src/app/command/bridge/bridged_command_ids.md) |
| calls | [metadata_for](/crates/oxide-app/src/keymap/catalog/mod/metadata_for.md) |
