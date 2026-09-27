---
okf_version: "0.2"
type: Function
title: pad_stack_preview
description: v0.20 — Pad Stack preview. CPU-side iso-projected 3D rendering of
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/pad_stack_preview
language: rust
---

# pad_stack_preview

v0.20 — Pad Stack preview. CPU-side iso-projected 3D rendering of

## Signature

```rust
pub(super) fn pad_stack_preview(values: &PadFormValues) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.20 — Pad Stack preview. CPU-side iso-projected 3D rendering of
the pad: copper top face (red), solder mask outset (blue) at the
board surface, and a hole punched through both for THT pads.
Uses a 60° camera tilt so the viewer sees the top face plus the
stack thickness as in Altium's PCB Library preview.

Mirrors the projection helper in `preview3d.rs` but for a single
centred pad — no body / courtyard / bbox math.

## Source
Lines 19–437 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stack_preview](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| calls | [project](/crates/oxide-app/src/app/state/scope/project.md) |
| called_by | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
