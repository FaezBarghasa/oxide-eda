---
okf_version: "0.2"
type: Class
title: RoleTag
description: v0.16.2 — role tag attached to a sketch entity. The Sketch-mode
resource: crates/oxide-app/src/library/messages/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/mod/RoleTag
language: rust
---

# RoleTag

v0.16.2 — role tag attached to a sketch entity. The Sketch-mode

## Signature

```rust
pub enum RoleTag
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

v0.16.2 — role tag attached to a sketch entity. The Sketch-mode
inspector emits one of these via
[`FootprintEditorMsg::SketchSetRole`]; the dispatcher
clears every `*Attr` slot on the target entity and writes the
matching one with sensible defaults. Bake auto-emits whatever
geometry the role implies (pad / silk segment / courtyard
polygon / mask opening / pour / paste aperture / keepout / board
cutout). `Pad` is only valid on a Point — non-Point entities
fall through as a silent no-op.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 367–398 in `crates/oxide-app/src/library/messages/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/messages/mod.md) |
