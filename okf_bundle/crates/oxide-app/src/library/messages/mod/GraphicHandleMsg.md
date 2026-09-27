---
okf_version: "0.2"
type: Class
title: GraphicHandleMsg
description: Resize-handle identity for a Symbol graphic — pure-data alias of
resource: crates/oxide-app/src/library/messages/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/mod/GraphicHandleMsg
language: rust
---

# GraphicHandleMsg

Resize-handle identity for a Symbol graphic — pure-data alias of

## Signature

```rust
pub enum GraphicHandleMsg
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Resize-handle identity for a Symbol graphic — pure-data alias of
`editor::symbol::state::GraphicHandle`. Carried by
`SymbolEditorMsg::MoveGraphicHandle` so the dispatcher
knows which handle of which graphic the canvas is dragging.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Source
Lines 482–500 in `crates/oxide-app/src/library/messages/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/messages/mod.md) |
