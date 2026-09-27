---
okf_version: "0.2"
type: Function
title: deleting_a_port_backed_sheet_pin_does_not_survive_reconcile
description: "[test]"
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/deleting_a_port_backed_sheet_pin_does_not_survive_reconcile
language: rust
---

# deleting_a_port_backed_sheet_pin_does_not_survive_reconcile

[test]

## Signature

```rust
fn deleting_a_port_backed_sheet_pin_does_not_survive_reconcile()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 554–594 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| calls | [test_sheet_pin](/crates/oxide-engine/src/lib/test_sheet_pin.md) |
| calls | [test_child_sheet](/crates/oxide-engine/src/lib/test_child_sheet.md) |
| calls | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
| calls | [reconcile_child_sheet_pins](/crates/oxide-engine/src/sheet/reconcile_child_sheet_pins.md) |
