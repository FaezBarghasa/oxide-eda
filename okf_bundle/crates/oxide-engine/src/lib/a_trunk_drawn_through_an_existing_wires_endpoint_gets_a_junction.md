---
okf_version: "0.2"
type: Function
title: a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction
description: "Drawing a stub and then a trunk through the stub's endpoint is the"
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction
language: rust
---

# a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction

Drawing a stub and then a trunk through the stub's endpoint is the

## Signature

```rust
fn a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction()
```

## Decorators

- `test`

## Docstring

Drawing a stub and then a trunk through the stub's endpoint is the
ordinary way a T gets drawn, and it used to leave no junction dot:
`needed_junction` only ever inspected the *new* wire's own two
endpoints. The netlist treats an undotted T as disconnected (issue
#107), so the connection was silently lost (issue #402).
[test]

## Source
Lines 642–658 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| calls | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
| calls | [wire](/crates/oxide-engine/src/lib/wire.md) |
