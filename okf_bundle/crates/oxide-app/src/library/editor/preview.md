---
okf_version: "0.2"
type: Module
title: preview
description: "Preview tab — read-only Symbol + Footprint render side-by-side,"
resource: crates/oxide-app/src/library/editor/preview.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/preview
language: rust
---

# preview

Preview tab — read-only Symbol + Footprint render side-by-side,

## Docstring

Preview tab — read-only Symbol + Footprint render side-by-side,
inline Pin Map subsection, Where-Used footer line.

The Preview surface is purely read-only for primitives;
right-click on either render → context menu fires
[`crate::library::messages::LibraryMessage::OpenPrimitiveEditor`]
to open the standalone `.snxsym` / `.snxfpt` document tab.

The Symbol render uses a minimal preview canvas — no pan/zoom, no
drag/drop, just an auto-fit body rectangle + pin stubs. Same for
the Footprint render: pad outlines on a flat board with no input
capture.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/editor/preview/view.md) |
| related | [header_row](/crates/oxide-app/src/library/editor/preview/header_row.md) |
| related | [render_panes](/crates/oxide-app/src/library/editor/preview/render_panes.md) |
| related | [render_pane](/crates/oxide-app/src/library/editor/preview/render_pane.md) |
| related | [symbol_summary_text](/crates/oxide-app/src/library/editor/preview/symbol_summary_text.md) |
| related | [footprint_summary_text](/crates/oxide-app/src/library/editor/preview/footprint_summary_text.md) |
| related | [open_btn](/crates/oxide-app/src/library/editor/preview/open_btn.md) |
| related | [symbol_path](/crates/oxide-app/src/library/editor/preview/symbol_path.md) |
| related | [footprint_path](/crates/oxide-app/src/library/editor/preview/footprint_path.md) |
| related | [pin_map_subsection](/crates/oxide-app/src/library/editor/preview/pin_map_subsection.md) |
| related | [override_action_btn](/crates/oxide-app/src/library/editor/preview/override_action_btn.md) |
| related | [override_editor_row](/crates/oxide-app/src/library/editor/preview/override_editor_row.md) |
| related | [where_used_footer](/crates/oxide-app/src/library/editor/preview/where_used_footer.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
