---
okf_version: "0.2"
type: Function
title: same
resource: crates/oxide-erc/src/rules/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/mod/same
language: rust
---

# same

## Signature

```rust
fn same(a: &Point, b: &Point) -> bool
```

## Source
Lines 27–29 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| called_by | [dangling_wire](/crates/oxide-erc/src/rules/mod/dangling_wire.md) |
| called_by | [power_port_short](/crates/oxide-erc/src/rules/mod/power_port_short.md) |
