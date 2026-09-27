---
okf_version: "0.2"
type: Function
title: view_footprint
description: "Render the standalone Footprint editor for a `.snxfpt` tab. Mirrors"
resource: crates/oxide-app/src/library/editor/standalone/footprint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/footprint/view_footprint
language: rust
---

# view_footprint

Render the standalone Footprint editor for a `.snxfpt` tab. Mirrors

## Signature

```rust
pub fn view_footprint(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
    _theme_id: oxide_types::theme::ThemeId,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the standalone Footprint editor for a `.snxfpt` tab. Mirrors
the in-Component Editor footprint surface (toolbar + canvas +
footer) but skips the right-column Body 3D / 3D preview / STEP
attach panel — those edit Component-level fields that live on the
Footprint primitive's `body_3d` and `step_attachment` slots and the
view tree for them is reused via the Component Editor surface.
Pure pad-layout standalone editing is what `.snxfpt` needs first.

## Source
Lines 25–101 in `crates/oxide-app/src/library/editor/standalone/footprint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-app/src/library/editor/standalone/footprint.md) |
| calls | [view_footprint_canvas](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_canvas.md) |
| calls | [view_footprint_footer](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_footer.md) |
| calls | [view_footprint_layers_strip](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_layers_strip.md) |
| calls | [mode_switcher_overlay](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/mode_switcher_overlay.md) |
| called_by | [view_center](/crates/oxide-app/src/app/view/mod/view_center.md) |
