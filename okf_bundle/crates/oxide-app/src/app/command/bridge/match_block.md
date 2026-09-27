---
okf_version: "0.2"
type: Function
title: match_block
description: "The `core_to_message` match block alone, cut out of `src`."
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/match_block
language: rust
---

# match_block

The `core_to_message` match block alone, cut out of `src`.

## Signature

```rust
fn match_block(src: &str) -> &str
```

## Docstring

The `core_to_message` match block alone, cut out of `src`.

Load-bearing: `BRIDGE_SRC` is the WHOLE file, tests included, and
this module now holds `UNMAPPED_CATALOG_IDS` — 64 string literals
on lines that start with `"`. Scanning the whole file would read
every one of them as a bridge arm and quietly invert the ratchet.

## Source
Lines 207–220 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [bridged_command_ids](/crates/oxide-app/src/app/command/bridge/bridged_command_ids.md) |
