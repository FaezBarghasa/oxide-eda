---
okf_version: "0.2"
type: Class
title: SketchEdit
description: One atomic edit to a sketch. Ordering matters — the dispatcher
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/SketchEdit
language: rust
---

# SketchEdit

One atomic edit to a sketch. Ordering matters — the dispatcher

## Signature

```rust
pub enum SketchEdit
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

One atomic edit to a sketch. Ordering matters — the dispatcher
applies them in arrival order so a follow-up solve sees the
post-edit `SketchData`.
[derive(Debug, Clone)]

## Methods

- `id`
- `dx`
- `dy`
- `name`
- `expr`
- `name`

## Source
Lines 22–63 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.md) |
