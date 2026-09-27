---
okf_version: "0.2"
type: Module
title: active_bar
description: Generic Active Bar — a floating row of icon buttons used by every
resource: crates/oxide-widgets/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/active_bar/mod
language: rust
---

# active_bar

Generic Active Bar — a floating row of icon buttons used by every

## Docstring

Generic Active Bar — a floating row of icon buttons used by every
Oxide editor surface (schematic, schematic library, PCB, PCB
library) to surface the primary place / select tools.

Altium-parity affordance: the bar floats over the canvas at the
top, the active tool reads with the accent background, disabled
tools (e.g. v0.9.x stubs) read greyed-out, and tooltips spell
out the tool name. Each editor builds its own
`Vec<ActiveBarItem<M>>` with the editor-specific message type;
this widget just renders.

Item variants:
- `Button(ActiveBarButton<M>)` — clickable tool button. Optional
right-press message + chevron indicator for buttons that open
dropdowns (the dropdown overlay itself is rendered separately
by the consumer as a Stack overlay layer).
- `Separator` — thin vertical divider between groups.
- `Custom(Element<M>)` — escape hatch for special slots (e.g.
the schematic editor's wire draw-mode 90°/45°/Any cycle pill).

# Example

```ignore
use oxide_widgets::active_bar::{view, ActiveBarItem, ActiveBarButton, ActiveBarIcon};

let items = vec![
ActiveBarItem::Button(ActiveBarButton {
icon: ActiveBarIcon::Svg(SELECT_SVG.clone()),
tooltip: "Select".into(),
enabled: true,
selected: matches!(tool, Tool::Select),
on_press: Some(Msg::SetTool(Tool::Select)),
on_right_press: Some(Msg::OpenSelectMenu),
dropdown_indicator: Some(ActiveBarIcon::Svg(CHEVRON_SVG.clone())),
}),
ActiveBarItem::Separator,
// …
];
view(items, &tokens)
```

Render the resulting `Element<M>` into a `Stack` overlay so it
floats over the canvas content.

## Relationships

| Type | Target |
|------|--------|
| related | [ActiveBarItem](/crates/oxide-widgets/src/active_bar/mod/ActiveBarItem.md) |
| related | [button](/crates/oxide-widgets/src/active_bar/mod/button.md) |
| related | [custom](/crates/oxide-widgets/src/active_bar/mod/custom.md) |
| related | [width](/crates/oxide-widgets/src/active_bar/mod/width.md) |
| related | [button](/crates/oxide-widgets/src/active_bar/mod/button.md) |
| related | [custom](/crates/oxide-widgets/src/active_bar/mod/custom.md) |
| related | [width](/crates/oxide-widgets/src/active_bar/mod/width.md) |
| related | [slot_offsets](/crates/oxide-widgets/src/active_bar/mod/slot_offsets.md) |
| related | [ActiveBarButton](/crates/oxide-widgets/src/active_bar/mod/ActiveBarButton.md) |
| related | [default](/crates/oxide-widgets/src/active_bar/mod/default.md) |
| related | [default](/crates/oxide-widgets/src/active_bar/mod/default.md) |
| related | [ActiveBarIcon](/crates/oxide-widgets/src/active_bar/mod/ActiveBarIcon.md) |
| related | [view_with_overlay](/crates/oxide-widgets/src/active_bar/mod/view_with_overlay.md) |
| related | [view](/crates/oxide-widgets/src/active_bar/mod/view.md) |
| related | [view_item](/crates/oxide-widgets/src/active_bar/mod/view_item.md) |
| related | [view_button](/crates/oxide-widgets/src/active_bar/mod/view_button.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
