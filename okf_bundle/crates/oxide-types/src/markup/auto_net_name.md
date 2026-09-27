---
okf_version: "0.2"
type: Function
title: auto_net_name
description: Default name for an unnamed net.
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/auto_net_name
language: rust
---

# auto_net_name

Default name for an unnamed net.

## Signature

```rust
pub fn auto_net_name(sheet: &str, pins: &[(String, String)]) -> Option<String>
```

## Visibility

- `pub`

## Docstring

Default name for an unnamed net.

Format: `unnamed-<sheet>:<ref>:<pin>`. Picks the lexicographically-
smallest `(refdes, pin)` for determinism. Sheet defaults to empty
string when the caller doesn't have a sheet context.

## Source
Lines 74–84 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
