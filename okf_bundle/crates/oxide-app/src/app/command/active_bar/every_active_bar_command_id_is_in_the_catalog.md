---
okf_version: "0.2"
type: Function
title: every_active_bar_command_id_is_in_the_catalog
description: Every id in the table must resolve in the command catalog —
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/every_active_bar_command_id_is_in_the_catalog
language: rust
---

# every_active_bar_command_id_is_in_the_catalog

Every id in the table must resolve in the command catalog —

## Signature

```rust
fn every_active_bar_command_id_is_in_the_catalog()
```

## Decorators

- `test`

## Docstring

Every id in the table must resolve in the command catalog —
otherwise it is a name the Keyboard Shortcuts pane cannot label and
a profile cannot meaningfully bind.
[test]

## Source
Lines 259–273 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
| calls | [command_ids](/crates/oxide-app/src/app/command/active_bar/command_ids.md) |
| calls | [metadata_for](/crates/oxide-app/src/keymap/catalog/mod/metadata_for.md) |
