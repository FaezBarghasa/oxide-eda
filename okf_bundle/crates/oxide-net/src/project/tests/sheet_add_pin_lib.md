---
okf_version: "0.2"
type: Function
title: sheet_add_pin_lib
description: "Library whose single pin sits at `local` (for the rotated-symbol case)."
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/sheet_add_pin_lib
language: rust
---

# sheet_add_pin_lib

Library whose single pin sits at `local` (for the rotated-symbol case).

## Signature

```rust
fn sheet_add_pin_lib(sheet: &mut SchematicSheet, lib_id: &str, local: Point)
```

## Docstring

Library whose single pin sits at `local` (for the rotated-symbol case).

## Source
Lines 324–347 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| called_by | [equivalence_gate_root_only](/crates/oxide-net/src/project/tests/equivalence_gate_root_only.md) |
