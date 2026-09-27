---
okf_version: "0.2"
type: Module
title: keys
description: "Keyboard handling — the `KeyPressed` branch of `Program::update`,"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/keys
language: rust
---

# keys

Keyboard handling — the `KeyPressed` branch of `Program::update`,

## Docstring

Keyboard handling — the `KeyPressed` branch of `Program::update`,
extracted verbatim. Escape cancels an in-progress multi-click draw;
Delete/Backspace, Home, Ctrl+A, Space (rotate), and undo/redo keep
identical key matching and publish sites.

## Relationships

| Type | Target |
|------|--------|
| related | [on_key_pressed](/crates/oxide-app/src/library/editor/symbol/canvas/input/keys/on_key_pressed.md) |
| related | [on_key_pressed](/crates/oxide-app/src/library/editor/symbol/canvas/input/keys/on_key_pressed.md) |
