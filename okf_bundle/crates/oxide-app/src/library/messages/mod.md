---
okf_version: "0.2"
type: Module
title: messages
description: Library subsystem message tree.
resource: crates/oxide-app/src/library/messages/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/mod
language: rust
---

# messages

Library subsystem message tree.

## Docstring

Library subsystem message tree.

Mirrors the existing `Message` → dispatcher → handler split used across
the rest of `oxide-app`. The top-level `LibraryMessage` is folded into
[`crate::app::contracts::Message::Library`]; each sub-enum routes to a
purpose-built handler.

Keep variants small and copy-cheap where possible — these messages
ride through the entire iced update tree, including for the multi-
window editor surface (one editor window per `ComponentId`).

## Relationships

| Type | Target |
|------|--------|
| related | [CloseLibraryChoice](/crates/oxide-app/src/library/messages/mod/CloseLibraryChoice.md) |
| related | [EditorMsg](/crates/oxide-app/src/library/messages/mod/EditorMsg.md) |
| related | [ParamKindMsg](/crates/oxide-app/src/library/messages/mod/ParamKindMsg.md) |
| related | [SymbolToolMsg](/crates/oxide-app/src/library/messages/mod/SymbolToolMsg.md) |
| related | [SketchConstraintTag](/crates/oxide-app/src/library/messages/mod/SketchConstraintTag.md) |
| related | [RoleTag](/crates/oxide-app/src/library/messages/mod/RoleTag.md) |
| related | [label](/crates/oxide-app/src/library/messages/mod/label.md) |
| related | [label](/crates/oxide-app/src/library/messages/mod/label.md) |
| related | [fmt](/crates/oxide-app/src/library/messages/mod/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/messages/mod/fmt.md) |
| related | [SymbolSelectionMsg](/crates/oxide-app/src/library/messages/mod/SymbolSelectionMsg.md) |
| related | [FieldKeyMsg](/crates/oxide-app/src/library/messages/mod/FieldKeyMsg.md) |
| related | [GraphicHandleMsg](/crates/oxide-app/src/library/messages/mod/GraphicHandleMsg.md) |
| related | [SymbolRotatePivotMsg](/crates/oxide-app/src/library/messages/mod/SymbolRotatePivotMsg.md) |
| related | [SymbolContextTargetMsg](/crates/oxide-app/src/library/messages/mod/SymbolContextTargetMsg.md) |
| related | [SymbolContextSubmenuMsg](/crates/oxide-app/src/library/messages/mod/SymbolContextSubmenuMsg.md) |
| related | [PrimitiveEdit](/crates/oxide-app/src/library/messages/mod/PrimitiveEdit.md) |
| related | [PickerMsg](/crates/oxide-app/src/library/messages/mod/PickerMsg.md) |
| related | [BrowserEditMsg](/crates/oxide-app/src/library/messages/mod/BrowserEditMsg.md) |
| related | [PrimitivePickerMsg](/crates/oxide-app/src/library/messages/mod/PrimitivePickerMsg.md) |
| related | [SettingsMsg](/crates/oxide-app/src/library/messages/mod/SettingsMsg.md) |
