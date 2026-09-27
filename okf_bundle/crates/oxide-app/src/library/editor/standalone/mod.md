---
okf_version: "0.2"
type: Module
title: standalone
description: "Standalone primitive editor tabs (`.snxsym` / `.snxfpt`)."
resource: crates/oxide-app/src/library/editor/standalone/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/standalone/mod
language: rust
---

# standalone

Standalone primitive editor tabs (`.snxsym` / `.snxfpt`).

## Docstring

Standalone primitive editor tabs (`.snxsym` / `.snxfpt`).

Renders `.snxsym` / `.snxfpt` files as main-window document tabs
using the canvas editor programs from
[`crate::library::editor::symbol`] and
[`crate::library::editor::footprint`] verbatim. Canvas events flow
through `LibraryMessage::PrimitiveEditorEvent { path, msg }` —
distinct from the Component Preview tab's
`LibraryMessage::EditorEvent { library_path, table, row_id, msg }`.

Primitives are first-class document tabs alongside `.snxsch` /
`.snxpcb`. Dirty-tracking is keyed by `path`; persistence is plain
JSON via serde. The Save action lives on the parent dispatcher
(`save_primitive_tab_at`); the editor view only owns the widget
tree.
