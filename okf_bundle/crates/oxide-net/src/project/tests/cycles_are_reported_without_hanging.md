---
okf_version: "0.2"
type: Function
title: cycles_are_reported_without_hanging
description: "7 ── Cycles: A→B→A and child-instantiates-root → SheetCycle, no hang."
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/cycles_are_reported_without_hanging
language: rust
---

# cycles_are_reported_without_hanging

7 ── Cycles: A→B→A and child-instantiates-root → SheetCycle, no hang.

## Signature

```rust
fn cycles_are_reported_without_hanging()
```

## Decorators

- `test`

## Docstring

7 ── Cycles: A→B→A and child-instantiates-root → SheetCycle, no hang.
[test]

## Source
Lines 619–639 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [child_sheet](/crates/oxide-net/src/project/tests/child_sheet.md) |
| calls | [stitch](/crates/oxide-net/src/project/tests/stitch.md) |
