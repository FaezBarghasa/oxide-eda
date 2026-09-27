---
okf_version: "0.2"
type: Function
title: symbol_with_pin
description: "A non-power symbol with a single pin, its `connected` flag set"
resource: crates/oxide-erc/src/rules/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/tests/symbol_with_pin
language: rust
---

# symbol_with_pin

A non-power symbol with a single pin, its `connected` flag set

## Signature

```rust
fn symbol_with_pin(reference: &str, world_pos: Point, connected: bool) -> ErcSymbol
```

## Docstring

A non-power symbol with a single pin, its `connected` flag set
exactly as `context::point_is_connected` would compute it — these
tests build `ErcContext` by hand (below the projection step), so the
flag is the fixture's job here.

## Source
Lines 62–77 in `crates/oxide-erc/src/rules/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-erc/src/rules/tests.md) |
