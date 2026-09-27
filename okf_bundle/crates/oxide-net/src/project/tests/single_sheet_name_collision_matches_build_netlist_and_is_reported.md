---
okf_version: "0.2"
type: Function
title: single_sheet_name_collision_matches_build_netlist_and_is_reported
description: "10b ── A bare single-sheet collision dedups through the same shared pass,"
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/single_sheet_name_collision_matches_build_netlist_and_is_reported
language: rust
---

# single_sheet_name_collision_matches_build_netlist_and_is_reported

10b ── A bare single-sheet collision dedups through the same shared pass,

## Signature

```rust
fn single_sheet_name_collision_matches_build_netlist_and_is_reported()
```

## Decorators

- `test`

## Docstring

10b ── A bare single-sheet collision dedups through the same shared pass,
so the root netlist stays byte-identical to build_netlist while the
stitcher still surfaces the clash.
[test]

## Source
Lines 759–790 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [wire](/crates/oxide-net/src/project/tests/wire.md) |
| calls | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| calls | [label](/crates/oxide-net/src/project/tests/label.md) |
| calls | [add_lib](/crates/oxide-net/src/project/tests/add_lib.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| calls | [stitch](/crates/oxide-net/src/project/tests/stitch.md) |
| calls | [names](/crates/oxide-net/src/project/tests/names.md) |
