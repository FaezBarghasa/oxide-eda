---
okf_version: "0.2"
type: Class
title: ToolPending
description: Transient per-tool gesture state. The canvas Program reads + writes
resource: crates/oxide-app/src/library/editor/footprint/state/tool.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/tool/ToolPending
language: rust
---

# ToolPending

Transient per-tool gesture state. The canvas Program reads + writes

## Signature

```rust
pub enum ToolPending
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Transient per-tool gesture state. The canvas Program reads + writes
it through editor messages so the iced update loop can persist it
across renders without coupling the canvas's internal `cstate`
(which is local to the canvas program) to the editor's serialised
state.
[derive(Debug, Clone, Default)]

## Methods

- `first`
- `first`
- `first`
- `center`
- `center`
- `center`
- `start`
- `start`
- `start`
- `end`
- `array_id`
- `first`
- `line`

## Source
Lines 102–156 in `crates/oxide-app/src/library/editor/footprint/state/tool.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool](/crates/oxide-app/src/library/editor/footprint/state/tool.md) |
