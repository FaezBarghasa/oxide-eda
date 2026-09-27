---
okf_version: "0.2"
type: Function
title: assert_equiv
description: "1 ── Equivalence gate: one root, empty resolved, is byte-for-byte"
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/assert_equiv
language: rust
---

# assert_equiv

1 ── Equivalence gate: one root, empty resolved, is byte-for-byte

## Signature

```rust
fn assert_equiv(sheet: &SchematicSheet)
```

## Docstring

1 ── Equivalence gate: one root, empty resolved, is byte-for-byte
build_netlist(root), incl. rotated/mirrored symbols.

## Source
Lines 268–280 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [stitch](/crates/oxide-net/src/project/tests/stitch.md) |
| called_by | [equivalence_gate_root_only](/crates/oxide-net/src/project/tests/equivalence_gate_root_only.md) |
