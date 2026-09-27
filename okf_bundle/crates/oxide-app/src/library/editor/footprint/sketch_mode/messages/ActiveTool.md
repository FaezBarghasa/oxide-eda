---
okf_version: "0.2"
type: Class
title: ActiveTool
description: "The active drawing tool inside Sketch mode. The Phase 6 UI's"
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/ActiveTool
language: rust
---

# ActiveTool

The active drawing tool inside Sketch mode. The Phase 6 UI's

## Signature

```rust
pub enum ActiveTool
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Default)`

## Visibility

- `pub`

## Docstring

The active drawing tool inside Sketch mode. The Phase 6 UI's
tool palette emits `SetTool(...)` to switch the active tool;
the canvas uses this to interpret pointer events. The placeholder
list maps to the SKETCH_MODE_v0.13_PLAN.md §6.3 spec.
[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]

## Source
Lines 79–95 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.md) |
