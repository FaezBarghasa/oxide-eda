---
okf_version: "0.2"
type: Module
title: tab_pill
description: Custom tab pill widget — three-sided border (top + sides) plus a
resource: crates/oxide-widgets/src/tab_pill.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/tab_pill
language: rust
---

# tab_pill

Custom tab pill widget — three-sided border (top + sides) plus a

## Docstring

Custom tab pill widget — three-sided border (top + sides) plus a
2-px accent strip below for the active marker.

Built as a real `iced::Widget` because iced 0.14's standard `Border`
is uniform on all four sides — we can't draw "top + sides only" via
the built-in container style. Trying to fake it with a stacked
accent-bg outer + rounded-top inner caused the accent colour to
bleed through the rounded corners; the widget below paints only the
three edges + accent strip directly via `renderer.fill_quad`.

Geometry:

```text
┌─────────────┐  ← top + side borders (1 px, `border` colour)
│             │
│   content   │  ← caller-supplied child (text + icons + ...)
│             │
└─────────────┘  ← bottom edge has NO border
▓▓▓▓▓▓▓▓▓▓▓▓▓   ← 2 px accent strip when `is_active`
```

The widget owns the bg fill so adjacent pills can sit flush with
zero spacing and still read as discrete tabs (the bg + side
borders provide the divider).

## Relationships

| Type | Target |
|------|--------|
| related | [AccentPosition](/crates/oxide-widgets/src/tab_pill/AccentPosition.md) |
| related | [TabPillStyle](/crates/oxide-widgets/src/tab_pill/TabPillStyle.md) |
| related | [TabPill](/crates/oxide-widgets/src/tab_pill/TabPill.md) |
| related | [new](/crates/oxide-widgets/src/tab_pill/new.md) |
| related | [new](/crates/oxide-widgets/src/tab_pill/new.md) |
| related | [tag](/crates/oxide-widgets/src/tab_pill/tag.md) |
| related | [state](/crates/oxide-widgets/src/tab_pill/state.md) |
| related | [children](/crates/oxide-widgets/src/tab_pill/children.md) |
| related | [diff](/crates/oxide-widgets/src/tab_pill/diff.md) |
| related | [size](/crates/oxide-widgets/src/tab_pill/size.md) |
| related | [layout](/crates/oxide-widgets/src/tab_pill/layout.md) |
| related | [operate](/crates/oxide-widgets/src/tab_pill/operate.md) |
| related | [update](/crates/oxide-widgets/src/tab_pill/update.md) |
| related | [mouse_interaction](/crates/oxide-widgets/src/tab_pill/mouse_interaction.md) |
| related | [draw](/crates/oxide-widgets/src/tab_pill/draw.md) |
| related | [overlay](/crates/oxide-widgets/src/tab_pill/overlay.md) |
| related | [tag](/crates/oxide-widgets/src/tab_pill/tag.md) |
| related | [state](/crates/oxide-widgets/src/tab_pill/state.md) |
| related | [children](/crates/oxide-widgets/src/tab_pill/children.md) |
| related | [diff](/crates/oxide-widgets/src/tab_pill/diff.md) |
| related | [size](/crates/oxide-widgets/src/tab_pill/size.md) |
| related | [layout](/crates/oxide-widgets/src/tab_pill/layout.md) |
| related | [operate](/crates/oxide-widgets/src/tab_pill/operate.md) |
| related | [update](/crates/oxide-widgets/src/tab_pill/update.md) |
| related | [mouse_interaction](/crates/oxide-widgets/src/tab_pill/mouse_interaction.md) |
| related | [draw](/crates/oxide-widgets/src/tab_pill/draw.md) |
| related | [overlay](/crates/oxide-widgets/src/tab_pill/overlay.md) |
| related | [from](/crates/oxide-widgets/src/tab_pill/from.md) |
| related | [from](/crates/oxide-widgets/src/tab_pill/from.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
