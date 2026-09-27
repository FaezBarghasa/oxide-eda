---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/apply
language: rust
---

# apply

## Signature

```rust
pub(in crate::library::editor::footprint::updates) fn apply(
    editor: &mut crate::app::FootprintEditorState,
    msg: FootprintEditorMsg,
)
```

## Visibility

- `pub(in crate::library::editor::footprint::updates)`

## Source
Lines 20–36 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entities](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.md) |
| calls | [place_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/place_point.md) |
| calls | [move_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_point.md) |
| calls | [move_line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_line.md) |
| calls | [resize_round_pad](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/resize_round_pad.md) |
