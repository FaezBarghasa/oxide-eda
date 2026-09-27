---
okf_version: "0.2"
type: Function
title: an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction
description: "A dot the netlist will not honour is worse than no dot: it asserts a"
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction
language: rust
---

# an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction

A dot the netlist will not honour is worse than no dot: it asserts a

## Signature

```rust
fn an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction()
```

## Decorators

- `test`

## Docstring

A dot the netlist will not honour is worse than no dot: it asserts a
connection the derivation refuses to make, with a reassuring visual.

The stub's endpoint sits 5 µm off the trunk — inside the 0.01 mm float
tolerance the geometry helpers use, but *not* exactly collinear in the
netlist's 1 µm key space, so `SheetConnectivity` would drop the dot and
leave the two wires on separate nets. Mint nothing rather than lie.
[test]

## Source
Lines 762–780 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| calls | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
| calls | [wire](/crates/oxide-engine/src/lib/wire.md) |
