---
okf_version: "0.2"
type: Function
title: unused_pin
description: "---------------------------------------------------------------------------"
resource: crates/oxide-erc/src/rules/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/mod/unused_pin
language: rust
---

# unused_pin

---------------------------------------------------------------------------

## Signature

```rust
pub(crate) fn unused_pin(ctx: &ErcContext, out: &mut Vec<Diagnostic>)
```

## Visibility

- `pub(crate)`

## Docstring

---------------------------------------------------------------------------
Rule: UnusedPin
---------------------------------------------------------------------------

## Source
Lines 73–101 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [sel](/crates/oxide-erc/src/lib/sel.md) |
| called_by | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
| called_by | [unused_pin_still_fires_when_not_connected](/crates/oxide-erc/src/rules/tests/unused_pin_still_fires_when_not_connected.md) |
| called_by | [unused_pin_trusts_a_through_junction_tap](/crates/oxide-erc/src/rules/tests/unused_pin_trusts_a_through_junction_tap.md) |
