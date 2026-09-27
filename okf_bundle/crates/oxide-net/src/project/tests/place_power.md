---
okf_version: "0.2"
type: Function
title: place_power
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/place_power
language: rust
---

# place_power

## Signature

```rust
fn place_power(sheet: &mut SchematicSheet, reference: &str, lib_id: &str, value: &str, at: Point)
```

## Source
Lines 147–152 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| called_by | [equivalence_gate_root_only](/crates/oxide-net/src/project/tests/equivalence_gate_root_only.md) |
| called_by | [two_flat_siblings_merge_by_shared_power_label_root_stays_separate](/crates/oxide-net/src/project/tests/multi_root/two_flat_siblings_merge_by_shared_power_label_root_stays_separate.md) |
| called_by | [power_symbol_and_power_label_merge_across_sheets](/crates/oxide-net/src/project/tests/power_symbol_and_power_label_merge_across_sheets.md) |
