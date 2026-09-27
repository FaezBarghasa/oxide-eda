---
okf_version: "0.2"
type: Function
title: symbol_context_target_to_msg
resource: crates/oxide-app/src/library/editor/standalone/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/symbol/symbol_context_target_to_msg
language: rust
---

# symbol_context_target_to_msg

## Signature

```rust
fn symbol_context_target_to_msg(
    target: sym_state::SymbolContextTarget,
) -> crate::library::messages::SymbolContextTargetMsg
```

## Source
Lines 358–368 in `crates/oxide-app/src/library/editor/standalone/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/library/editor/standalone/symbol.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| called_by | [symbol_action_to_primitive_msg](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_action_to_primitive_msg.md) |
