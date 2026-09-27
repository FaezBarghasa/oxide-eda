---
okf_version: "0.2"
type: Function
title: fingerprint
description: "Violations reduced to an order-insensitive, uuid-free fingerprint. Rule"
resource: crates/oxide-erc/tests/wire_order_determinism.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/tests/wire_order_determinism/fingerprint
language: rust
---

# fingerprint

Violations reduced to an order-insensitive, uuid-free fingerprint. Rule

## Signature

```rust
fn fingerprint(violations: &[Violation]) -> Vec<(RuleKind, String)>
```

## Docstring

Violations reduced to an order-insensitive, uuid-free fingerprint. Rule
*order* within the list legitimately follows document order; what must not
change is the set of verdicts.

## Source
Lines 73–83 in `crates/oxide-erc/tests/wire_order_determinism.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire_order_determinism](/crates/oxide-erc/tests/wire_order_determinism.md) |
