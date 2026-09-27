---
okf_version: "0.2"
type: Class
title: CommandFlags
description: GUI/undo/visibility flags for a command.
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/CommandFlags
language: rust
---

# CommandFlags

GUI/undo/visibility flags for a command.

## Signature

```rust
pub struct CommandFlags
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Default)`

## Visibility

- `pub`

## Docstring

GUI/undo/visibility flags for a command.
[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]

## Methods

- `gui_only`
- `mutates_doc`
- `undoable`
- `hidden`

## Source
Lines 94–104 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
