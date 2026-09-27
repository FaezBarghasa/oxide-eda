---
okf_version: "0.2"
type: Function
title: same_filename_children_of_different_parents_stitch_from_their_own_files
description: "12 ── #466: two parents in different directories reference a child by the"
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/same_filename_children_of_different_parents_stitch_from_their_own_files
language: rust
---

# same_filename_children_of_different_parents_stitch_from_their_own_files

12 ── #466: two parents in different directories reference a child by the

## Signature

```rust
fn same_filename_children_of_different_parents_stitch_from_their_own_files()
```

## Decorators

- `test`

## Docstring

12 ── #466: two parents in different directories reference a child by the
SAME bare filename ("power.sch"); each must resolve and stitch from its
OWN file. This is a real `ProjectGraph` built by hand (not the `stitch`
flat-namespace helper above), because the flat, one-slot-per-filename
model is exactly what #466 removes.
[test]

## Source
Lines 862–950 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [child_sheet](/crates/oxide-net/src/project/tests/child_sheet.md) |
| calls | [wire](/crates/oxide-net/src/project/tests/wire.md) |
| calls | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| calls | [label](/crates/oxide-net/src/project/tests/label.md) |
| calls | [add_lib](/crates/oxide-net/src/project/tests/add_lib.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| calls | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
| calls | [names](/crates/oxide-net/src/project/tests/names.md) |
