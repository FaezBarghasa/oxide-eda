---
okf_version: "0.2"
type: Function
title: orphan_label
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
concept_id: crates/oxide-erc/src/rules/mod/orphan_label
language: rust
---

# orphan_label

---------------------------------------------------------------------------

## Signature

```rust
pub(crate) fn orphan_label(ctx: &ErcContext, out: &mut Vec<Diagnostic>)
```

## Visibility

- `pub(crate)`

## Docstring

---------------------------------------------------------------------------
Rule: OrphanLabel
---------------------------------------------------------------------------

## Source
Lines 276–312 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| calls | [wire_pairs](/crates/oxide-erc/src/rules/mod/wire_pairs.md) |
| calls | [on_any_wire](/crates/oxide-erc/src/rules/mod/on_any_wire.md) |
| calls | [on_any_bus_endpoint](/crates/oxide-erc/src/rules/mod/on_any_bus_endpoint.md) |
| calls | [pt_key](/crates/oxide-erc/src/context/pt_key.md) |
| calls | [sel](/crates/oxide-erc/src/lib/sel.md) |
| called_by | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
| called_by | [orphan_label_accepts_a_power_label_on_a_wire_interior](/crates/oxide-erc/src/rules/tests/orphan_label_accepts_a_power_label_on_a_wire_interior.md) |
