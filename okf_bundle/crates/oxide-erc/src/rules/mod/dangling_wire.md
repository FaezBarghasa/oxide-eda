---
okf_version: "0.2"
type: Function
title: dangling_wire
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
concept_id: crates/oxide-erc/src/rules/mod/dangling_wire
language: rust
---

# dangling_wire

---------------------------------------------------------------------------

## Signature

```rust
pub(crate) fn dangling_wire(ctx: &ErcContext, out: &mut Vec<Diagnostic>)
```

## Visibility

- `pub(crate)`

## Docstring

---------------------------------------------------------------------------
Rule: DanglingWire
---------------------------------------------------------------------------

## Source
Lines 175–208 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [same](/crates/oxide-erc/src/rules/mod/same.md) |
| calls | [sel](/crates/oxide-erc/src/lib/sel.md) |
| called_by | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
