---
okf_version: "0.2"
type: Function
title: read_pin_matrix_overrides
description: "Read the pin-connection matrix overrides. Keys stored as `\"row,col\"`"
resource: crates/oxide-app/src/fonts/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/fonts/erc/read_pin_matrix_overrides
language: rust
---

# read_pin_matrix_overrides

Read the pin-connection matrix overrides. Keys stored as `"row,col"`

## Signature

```rust
pub fn read_pin_matrix_overrides() -> std::collections::HashMap<(u8, u8), oxide_erc::Severity>
```

## Visibility

- `pub`

## Docstring

Read the pin-connection matrix overrides. Keys stored as `"row,col"`
strings and values as the same severity strings as ERC overrides.

## Source
Lines 110–138 in `crates/oxide-app/src/fonts/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/oxide-app/src/fonts/erc.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
