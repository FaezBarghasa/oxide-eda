---
okf_version: "0.2"
type: Module
title: editor
description: Component Preview tab — read-only Symbol/Footprint render plus
resource: crates/oxide-app/src/library/editor/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/mod
language: rust
---

# editor

Component Preview tab — read-only Symbol/Footprint render plus

## Docstring

Component Preview tab — read-only Symbol/Footprint render plus
template-validated forms for parameters / supply / datasheet /
simulation.

In the v0.9-refactor-2 (DBLib) model, components are TSV rows
addressed by [`crate::library::state::EditorAddress`]
(`library_path + table + row_id`). The state lives on
[`crate::library::state::ComponentPreviewState`].

The Component view is preview-only: Symbol and Footprint render
read-only. Editing happens via standalone `.snxsym` / `.snxfpt`
document tabs in the main window — right-click on either render
in the Preview tab fires
[`crate::library::messages::LibraryMessage::OpenPrimitiveEditor`]
to open the standalone editor.

Tab count is 5: Preview / Parameters / Supply / Datasheet /
Simulation. Pin Map folds into Preview as an inline subsection;
History is reachable via `git log`; Where-Used is a footer line
on Preview.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/editor/mod/view.md) |
| related | [view_header](/crates/oxide-app/src/library/editor/mod/view_header.md) |
| related | [view_tabs](/crates/oxide-app/src/library/editor/mod/view_tabs.md) |
| related | [view_active_tab](/crates/oxide-app/src/library/editor/mod/view_active_tab.md) |
| related | [view_footer](/crates/oxide-app/src/library/editor/mod/view_footer.md) |
| related | [close_btn](/crates/oxide-app/src/library/editor/mod/close_btn.md) |
| related | [placeholder_card](/crates/oxide-app/src/library/editor/mod/placeholder_card.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
