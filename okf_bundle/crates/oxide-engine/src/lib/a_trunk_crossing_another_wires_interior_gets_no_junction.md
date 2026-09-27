---
okf_version: "0.2"
type: Function
title: a_trunk_crossing_another_wires_interior_gets_no_junction
description: "The negative twin: a trunk merely *crossing* another wire's interior is"
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/a_trunk_crossing_another_wires_interior_gets_no_junction
language: rust
---

# a_trunk_crossing_another_wires_interior_gets_no_junction

The negative twin: a trunk merely *crossing* another wire's interior is

## Signature

```rust
fn a_trunk_crossing_another_wires_interior_gets_no_junction()
```

## Decorators

- `test`

## Docstring

The negative twin: a trunk merely *crossing* another wire's interior is
not a connection, so it must not mint a dot.
[test]

## Source
Lines 663–681 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| calls | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
| calls | [wire](/crates/oxide-engine/src/lib/wire.md) |
