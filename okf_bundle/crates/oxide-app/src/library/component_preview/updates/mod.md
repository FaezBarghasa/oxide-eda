---
okf_version: "0.2"
type: Module
title: updates
description: Update logic for the Component Preview inline-edit surface.
resource: crates/oxide-app/src/library/component_preview/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/mod
language: rust
---

# updates

Update logic for the Component Preview inline-edit surface.

## Docstring

Update logic for the Component Preview inline-edit surface.

[`apply_inline_edit`] is a thin routing table: each [`EditorMsg`] that
mutates the previewed component row is dispatched to the concern
module that owns the affected field group — [`datasheet`], [`pin_map`],
[`supply`], [`parameters`], or [`sim`]. Trivial component-level
setters are handled inline.

The long tail of variants belonging to the standalone symbol/footprint
canvases is matched explicitly (rather than with a `_` wildcard) so a
newly added [`EditorMsg`] variant is a compile error here until it is
deliberately routed or ignored.

## Relationships

| Type | Target |
|------|--------|
| related | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
