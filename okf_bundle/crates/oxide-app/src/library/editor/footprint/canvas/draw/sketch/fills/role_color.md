---
okf_version: "0.2"
type: Function
title: role_color
description: v0.16.2 — pick a fill colour for a loop by inspecting each
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/role_color
language: rust
---

# role_color

v0.16.2 — pick a fill colour for a loop by inspecting each

## Signature

```rust
fn role_color(entity: &Entity) -> Option<FpLayer>
```

## Docstring

v0.16.2 — pick a fill colour for a loop by inspecting each
entity's role attr. Returns `None` when no entity in the loop
carries a role; the caller falls back to neutral grey.

## Source
Lines 175–224 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fills](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.md) |
