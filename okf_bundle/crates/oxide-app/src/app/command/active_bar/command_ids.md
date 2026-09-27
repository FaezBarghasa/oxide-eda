---
okf_version: "0.2"
type: Function
title: command_ids
description: "Every id this table carries. The bridge's coverage guard folds these"
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/command_ids
language: rust
---

# command_ids

Every id this table carries. The bridge's coverage guard folds these

## Signature

```rust
pub(crate) fn command_ids() -> impl Iterator<Item = &'static str>
```

## Decorators

- `cfg(test)`

## Visibility

- `pub(crate)`

## Docstring

Every id this table carries. The bridge's coverage guard folds these
into the set of ids `core_to_message` can resolve.

Test-gated because the guard is its only caller today. It comes out
of `cfg(test)` the moment a production consumer needs it — the
command palette listing Active Bar commands (#366) is the obvious one.
[cfg(test)]

## Source
Lines 212–214 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
| called_by | [every_active_bar_command_id_is_in_the_catalog](/crates/oxide-app/src/app/command/active_bar/every_active_bar_command_id_is_in_the_catalog.md) |
| called_by | [the_table_has_no_duplicates](/crates/oxide-app/src/app/command/active_bar/the_table_has_no_duplicates.md) |
| called_by | [resolvable_command_ids](/crates/oxide-app/src/app/command/bridge/resolvable_command_ids.md) |
