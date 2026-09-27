---
okf_version: "0.2"
type: Function
title: sym_editor_select_pin
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_pin_1
language: rust
---

# sym_editor_select_pin

## Signature

```rust
pub(super) fn sym_editor_select_pin(&mut self, pin_idx: usize) -> bool
```

## Visibility

- `pub(super)`

## Source
Lines 301–314 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
