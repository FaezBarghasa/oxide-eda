---
okf_version: "0.2"
type: Class
title: PrimitiveEdit
description: "A canvas-editor mutation, namespaced by surface. The former flat"
resource: crates/oxide-app/src/library/messages/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/mod/PrimitiveEdit
language: rust
---

# PrimitiveEdit

A canvas-editor mutation, namespaced by surface. The former flat

## Signature

```rust
pub enum PrimitiveEdit
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

A canvas-editor mutation, namespaced by surface. The former flat
`PrimitiveEditorMsg` (134 variants) is split into per-surface enums
([`FootprintEditorMsg`] / [`SymbolEditorMsg`], ADR-0001 D3); `Save` is
shared. Carried by [`LibraryMessage::PrimitiveEditorEvent`]; path-keyed
dispatch (`handle_primitive_editor_event`) routes each surface to the
matching editor state per the active tab's [`crate::app::TabKind`].
[derive(Debug, Clone)]

## Source
Lines 540–548 in `crates/oxide-app/src/library/messages/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/messages/mod.md) |
