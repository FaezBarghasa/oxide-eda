---
okf_version: "0.2"
type: Function
title: run
description: Run every enabled rule against the snapshot. Returns a flat list of
resource: crates/oxide-erc/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc/src/lib/run
language: rust
---

# run

Run every enabled rule against the snapshot. Returns a flat list of

## Signature

```rust
pub fn run(snapshot: &SchematicSheet) -> Vec<Violation>
```

## Visibility

- `pub`

## Docstring

Run every enabled rule against the snapshot. Returns a flat list of
violations in rule order.

## Source
Lines 131–137 in `crates/oxide-erc/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc/src/lib.md) |
| calls | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
| called_by | [erc_verdict_is_independent_of_wire_order_at_a_junction_less_t](/crates/oxide-erc/tests/wire_order_determinism/erc_verdict_is_independent_of_wire_order_at_a_junction_less_t.md) |
| called_by | [net_label_conflict_count_is_independent_of_wire_order](/crates/oxide-erc/tests/wire_order_determinism/net_label_conflict_count_is_independent_of_wire_order.md) |
