---
okf_version: "0.2"
type: Function
title: handle_print_preview_pan_start
description: Press on the preview viewport — arms pan-drag. Subsequent
resource: crates/oxide-app/src/app/handlers/menu/export/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_pan_start
language: rust
---

# handle_print_preview_pan_start

Press on the preview viewport — arms pan-drag. Subsequent

## Signature

```rust
impl Oxide { pub(crate) fn handle_print_preview_pan_start(&mut self) }
```

## Visibility

- `pub(crate)`

## Docstring

Press on the preview viewport — arms pan-drag. Subsequent
`last_mouse_pos` updates are converted into pan offsets in
`handle_layout_drag_moved`. Snaps to the current pan as the
origin so the page doesn't jump on press. Reads the cursor
from `interaction_state` rather than from the message so the
press location matches what `last_mouse_pos` says now (iced
builds messages eagerly at view-render time, so coords on
the message would be up to one frame stale).

## Source
Lines 424–434 in `crates/oxide-app/src/app/handlers/menu/export/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/handlers/menu/export/print_preview.md) |
