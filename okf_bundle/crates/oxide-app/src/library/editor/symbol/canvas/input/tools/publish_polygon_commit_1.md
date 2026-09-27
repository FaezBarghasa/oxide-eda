---
okf_version: "0.2"
type: Function
title: publish_polygon_commit
description: Publish the commit action + reset the ephemeral gesture-timing
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/tools/publish_polygon_commit_1
language: rust
---

# publish_polygon_commit

Publish the commit action + reset the ephemeral gesture-timing

## Signature

```rust
fn publish_polygon_commit(&self, state: &mut CanvasState) -> canvas::Action<CanvasAction>
```

## Docstring

Publish the commit action + reset the ephemeral gesture-timing
fields that live in `CanvasState`. The vertex-count / validity
check happens at the dispatcher (`SymbolEditorMsg::PolygonCommit`
handler) against the editor-owned stash, not here.

## Source
Lines 309–314 in `crates/oxide-app/src/library/editor/symbol/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools.md) |
