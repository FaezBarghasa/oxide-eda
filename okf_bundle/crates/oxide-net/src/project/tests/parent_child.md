---
okf_version: "0.2"
type: Function
title: parent_child
description: A parent with one child; the child has a labelled pin net. Returns
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/parent_child
language: rust
---

# parent_child

A parent with one child; the child has a labelled pin net. Returns

## Signature

```rust
fn parent_child(
    pin_name: &str,
    pin_pos: Point,
    child_label: &str,
    child_label_type: LabelType,
) -> (SchematicSheet, HashMap<String, SchematicSheet>)
```

## Docstring

A parent with one child; the child has a labelled pin net. Returns
(root, children map) for the binding tests.

## Source
Lines 351–380 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [wire](/crates/oxide-net/src/project/tests/wire.md) |
| calls | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| calls | [add_lib](/crates/oxide-net/src/project/tests/add_lib.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| calls | [child_sheet](/crates/oxide-net/src/project/tests/child_sheet.md) |
| calls | [label](/crates/oxide-net/src/project/tests/label.md) |
| called_by | [sheet_pin_binds_global_child_label](/crates/oxide-net/src/project/tests/sheet_pin_binds_global_child_label.md) |
| called_by | [sheet_pin_binds_hierarchical_child_label](/crates/oxide-net/src/project/tests/sheet_pin_binds_hierarchical_child_label.md) |
| called_by | [unmatched_sheet_pin_stays_local](/crates/oxide-net/src/project/tests/unmatched_sheet_pin_stays_local.md) |
