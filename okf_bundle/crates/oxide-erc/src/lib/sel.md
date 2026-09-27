---
okf_version: "0.2"
type: Function
title: sel
description: "Helper for rules to build a [`SelectedItem`] when they only have a uuid."
resource: crates/oxide-erc/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc/src/lib/sel
language: rust
---

# sel

Helper for rules to build a [`SelectedItem`] when they only have a uuid.

## Signature

```rust
pub(crate) fn sel(uuid: uuid::Uuid, kind: SelectedKind) -> SelectedItem
```

## Visibility

- `pub(crate)`

## Docstring

Helper for rules to build a [`SelectedItem`] when they only have a uuid.

## Source
Lines 186–188 in `crates/oxide-erc/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc/src/lib.md) |
| called_by | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ambiguous_label_anchor.md) |
| called_by | [bad_hier_sheet_pin](/crates/oxide-erc/src/rules/mod/bad_hier_sheet_pin.md) |
| called_by | [bus_bit_width_mismatch](/crates/oxide-erc/src/rules/mod/bus_bit_width_mismatch.md) |
| called_by | [dangling_wire](/crates/oxide-erc/src/rules/mod/dangling_wire.md) |
| called_by | [duplicate_ref_designator](/crates/oxide-erc/src/rules/mod/duplicate_ref_designator.md) |
| called_by | [hier_port_disconnected](/crates/oxide-erc/src/rules/mod/hier_port_disconnected.md) |
| called_by | [missing_power_flag](/crates/oxide-erc/src/rules/mod/missing_power_flag.md) |
| called_by | [net_label_conflict](/crates/oxide-erc/src/rules/mod/net_label_conflict.md) |
| called_by | [orphan_label](/crates/oxide-erc/src/rules/mod/orphan_label.md) |
| called_by | [power_port_short](/crates/oxide-erc/src/rules/mod/power_port_short.md) |
| called_by | [symbol_outside_sheet](/crates/oxide-erc/src/rules/mod/symbol_outside_sheet.md) |
| called_by | [unused_pin](/crates/oxide-erc/src/rules/mod/unused_pin.md) |
