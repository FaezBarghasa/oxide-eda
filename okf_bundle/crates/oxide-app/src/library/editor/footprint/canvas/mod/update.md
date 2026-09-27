---
okf_version: "0.2"
type: Function
title: update
resource: crates/oxide-app/src/library/editor/footprint/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/mod/update
language: rust
---

# update

## Signature

```rust
impl FootprintCanvas<'a> { fn update(
        &self,
        cstate: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Type Parameters

- `'a`

## Source
Lines 229–274 in `crates/oxide-app/src/library/editor/footprint/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/footprint/canvas/mod.md) |
