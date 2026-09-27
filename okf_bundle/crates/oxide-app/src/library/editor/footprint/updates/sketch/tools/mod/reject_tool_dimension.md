---
okf_version: "0.2"
type: Function
title: reject_tool_dimension
description: "Report a dimension buffer that cannot be read as a positive length,"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/reject_tool_dimension
language: rust
---

# reject_tool_dimension

Report a dimension buffer that cannot be read as a positive length,

## Signature

```rust
pub(super) fn reject_tool_dimension(
    editor: &mut crate::app::FootprintEditorState,
    tool: &'static str,
    field: &'static str,
    buffer: &str,
)
```

## Visibility

- `pub(super)`

## Docstring

Report a dimension buffer that cannot be read as a positive length,
and abort the gesture.

GH #599 — the tool creates nothing here. `tool` names the gesture
("Offset", "Fillet") and `field` the dimension ("offset distance",
"fillet radius") so both the Messages panel entry and the sketch
inspector's warning list say which input was refused and why.

## Source
Lines 133–154 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
| called_by | [fillet](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet.md) |
| called_by | [offset](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset.md) |
