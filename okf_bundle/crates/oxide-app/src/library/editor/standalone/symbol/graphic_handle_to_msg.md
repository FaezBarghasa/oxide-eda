---
okf_version: "0.2"
type: Function
title: graphic_handle_to_msg
resource: crates/oxide-app/src/library/editor/standalone/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/symbol/graphic_handle_to_msg
language: rust
---

# graphic_handle_to_msg

## Signature

```rust
fn graphic_handle_to_msg(handle: sym_state::GraphicHandle) -> GraphicHandleMsg
```

## Source
Lines 370–382 in `crates/oxide-app/src/library/editor/standalone/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/library/editor/standalone/symbol.md) |
| calls | [PolygonVertex](/crates/oxide-gfx/src/pipeline/polygon/PolygonVertex.md) |
| called_by | [symbol_action_to_primitive_msg](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_action_to_primitive_msg.md) |
