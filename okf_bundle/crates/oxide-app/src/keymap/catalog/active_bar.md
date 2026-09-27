---
okf_version: "0.2"
type: Module
title: active_bar
description: "Command metadata for the schematic Active Bar's actions."
resource: crates/oxide-app/src/keymap/catalog/active_bar.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/catalog/active_bar
language: rust
---

# active_bar

Command metadata for the schematic Active Bar's actions.

## Docstring

Command metadata for the schematic Active Bar's actions.

Split out of `schematic.rs` rather than appended to it: the Active Bar
contributes more commands than every other schematic surface combined,
and `schematic.rs` would land near the 1000-line file cap. Entries are
`CommandGroup::Schematic` all the same — the file boundary is about
size, not grouping.

The parameterised families are deliberately absent: `PlacePower*` (11),
`NetColor*` (8) and the four selection-arm modes each enumerate a value
domain and want one command plus an argument, which needs the
`CommandArgs` boundary (#367 / #366). Minting 23 frozen ids for them now
would be 23 names to deprecate later. See
`docs/audit/command-registry-action-surface-2026-07-25.md` §4.
