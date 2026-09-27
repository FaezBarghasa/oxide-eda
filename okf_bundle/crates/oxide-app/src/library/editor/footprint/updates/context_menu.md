---
okf_version: "0.2"
type: Module
title: context_menu
description: Footprint editor — context_menu update logic.
resource: crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/context_menu
language: rust
---

# context_menu

Footprint editor — context_menu update logic.

## Docstring

Footprint editor — context_menu update logic.

Split out of `apply_footprint_primitive_edit` per ADR-0001 D1/D2.
`apply` is a thin router; each `FootprintEditorMsg` variant delegates
to one named per-action fn below (object→action, ADR-0001 D2).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/apply.md) |
| related | [show](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/show.md) |
| related | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| related | [open_submenu](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/open_submenu.md) |
| related | [run_action](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/run_action.md) |
