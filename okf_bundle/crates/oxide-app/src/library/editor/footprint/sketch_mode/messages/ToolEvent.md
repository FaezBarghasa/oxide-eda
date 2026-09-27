---
okf_version: "0.2"
type: Class
title: ToolEvent
description: Pointer / keyboard events forwarded from the canvas into the
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/ToolEvent
language: rust
---

# ToolEvent

Pointer / keyboard events forwarded from the canvas into the

## Signature

```rust
pub enum ToolEvent
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

Pointer / keyboard events forwarded from the canvas into the
active tool. Phase 6 UI fills these in; v0.13 schema lands here so
the `SketchModeMsg::ToolEvent` arm can route them.
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `x_mm`
- `y_mm`
- `x_mm`
- `y_mm`
- `x_mm`
- `y_mm`

## Source
Lines 101–110 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.md) |
