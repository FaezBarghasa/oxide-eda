---
okf_version: "0.2"
type: Class
title: GraphicFillPicker
description: "Transient open-state for a placed graphic's fill colour-picker."
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/GraphicFillPicker
language: rust
---

# GraphicFillPicker

Transient open-state for a placed graphic's fill colour-picker.

## Signature

```rust
pub struct GraphicFillPicker
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Transient open-state for a placed graphic's fill colour-picker.
`idx` is the graphic's index in the active symbol; `advanced` is
`true` once the user expanded the inline palette into the HSV / RGB
overlay. UI-only — never serialized, never snapshotted for undo.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `idx`
- `advanced`

## Source
Lines 215–218 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
