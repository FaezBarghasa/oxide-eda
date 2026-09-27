---
okf_version: "0.2"
type: Function
title: translate_footprint_canvas_msg
description: Translate a message emitted by the footprint canvas widget
resource: crates/oxide-app/src/library/editor/standalone/footprint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/footprint/translate_footprint_canvas_msg
language: rust
---

# translate_footprint_canvas_msg

Translate a message emitted by the footprint canvas widget

## Signature

```rust
fn translate_footprint_canvas_msg(msg: LibraryMessage, path: &Path) -> LibraryMessage
```

## Docstring

Translate a message emitted by the footprint canvas widget
(`canvas::Program<LibraryMessage>` — see `canvas/mod.rs`) into the
standalone `.snxfpt` tab's primitive-editor envelope. The canvas
only ever emits `EditorEvent { msg: EditorMsg::Footprint(_), .. }`
(every message it builds goes through that wrapper — see e.g.
`canvas/input/tools.rs`'s `select_msg`), so that shape becomes a
real, path-carrying `PrimitiveEditorEvent`. This routing decision
used to live inline in the `.map()` closure at the call site, where
nothing could unit-test it; it's a free function here so a
regression (a special-cased variant routed to `PrimitiveEdit::Save`
ahead of the general arm) is directly testable.

## Source
Lines 677–702 in `crates/oxide-app/src/library/editor/standalone/footprint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-app/src/library/editor/standalone/footprint.md) |
| called_by | [sketch_placement_char_routes_to_footprint_uniformly](/crates/oxide-app/src/library/editor/standalone/footprint/sketch_placement_char_routes_to_footprint_uniformly.md) |
| called_by | [sketch_placement_tab_routes_to_footprint_not_save](/crates/oxide-app/src/library/editor/standalone/footprint/sketch_placement_tab_routes_to_footprint_not_save.md) |
| called_by | [view_footprint_canvas](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_canvas.md) |
