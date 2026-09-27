---
okf_version: "0.2"
type: Function
title: clear_ghost_previews
description: Clear every cursor-following ghost preview. Call before arming a new
resource: crates/oxide-app/src/app/actions.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/actions/clear_ghost_previews
language: rust
---

# clear_ghost_previews

Clear every cursor-following ghost preview. Call before arming a new

## Signature

```rust
impl Oxide { pub(crate) fn clear_ghost_previews(&mut self) }
```

## Visibility

- `pub(crate)`

## Docstring

Clear every cursor-following ghost preview. Call before arming a new
ghost or switching to a tool that doesn't have one, so a previously
armed ghost from another tool doesn't linger on the canvas.

## Source
Lines 7–11 in `crates/oxide-app/src/app/actions.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actions](/crates/oxide-app/src/app/actions.md) |
