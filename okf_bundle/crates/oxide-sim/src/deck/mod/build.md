---
okf_version: "0.2"
type: Function
title: build
description: Build the full PSpice netlist deck as a string.
resource: crates/oxide-sim/src/deck/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:49:22Z"
concept_id: crates/oxide-sim/src/deck/mod/build
language: rust
---

# build

Build the full PSpice netlist deck as a string.

## Signature

```rust
impl PSpiceDeckBuilder<'a> { pub fn build(&self) -> String }
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Build the full PSpice netlist deck as a string.

## Source
Lines 43–257 in `crates/oxide-sim/src/deck/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [deck](/crates/oxide-sim/src/deck/mod.md) |
| calls | [sanitize_spice_value](/crates/oxide-sim/src/deck/mod/sanitize_spice_value.md) |
| calls | [sanitize_identifier](/crates/oxide-sim/src/deck/mod/sanitize_identifier.md) |
| calls | [sanitize_node_name](/crates/oxide-sim/src/deck/mod/sanitize_node_name.md) |
