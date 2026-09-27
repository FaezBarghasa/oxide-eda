---
okf_version: "0.2"
type: Function
title: bad_hier_sheet_pin
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
concept_id: crates/oxide-erc/src/rules/mod/bad_hier_sheet_pin
language: rust
---

# bad_hier_sheet_pin

---------------------------------------------------------------------------

## Signature

```rust
pub(crate) fn bad_hier_sheet_pin(ctx: &ErcContext, out: &mut Vec<Diagnostic>)
```

## Visibility

- `pub(crate)`

## Docstring

---------------------------------------------------------------------------
Rule: BadHierSheetPin
---------------------------------------------------------------------------

## Source
Lines 420–510 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [sel](/crates/oxide-erc/src/lib/sel.md) |
| called_by | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
