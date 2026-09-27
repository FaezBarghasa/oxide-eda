---
okf_version: "0.2"
type: Module
title: messages
description: "Phase 5.5 — `SketchEdit` / `SketchModeMsg` enums consumed by the"
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages
language: rust
---

# messages

Phase 5.5 — `SketchEdit` / `SketchModeMsg` enums consumed by the

## Docstring

Phase 5.5 — `SketchEdit` / `SketchModeMsg` enums consumed by the
solve-on-edit dispatcher and emitted by the Phase 6 UI shell.

Designed to be plain-data: the iced `update` path matches on
`SketchModeMsg` and dispatches to
[`super::super::sketch_dispatch::apply_sketch_edit`] for the
`Edit(_)` variant. Tool-state changes are local to the editor
state.

## Relationships

| Type | Target |
|------|--------|
| related | [SketchEdit](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/SketchEdit.md) |
| related | [add_entity](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/add_entity.md) |
| related | [add_entity](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/add_entity.md) |
| related | [ActiveTool](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/ActiveTool.md) |
| related | [ToolEvent](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/ToolEvent.md) |
| related | [SketchModeMsg](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/SketchModeMsg.md) |
