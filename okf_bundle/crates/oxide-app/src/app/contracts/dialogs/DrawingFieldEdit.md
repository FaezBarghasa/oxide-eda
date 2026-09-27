---
okf_version: "0.2"
type: Class
title: DrawingFieldEdit
description: Per-shape edit descriptor. The Properties panel dispatches one of
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/DrawingFieldEdit
language: rust
---

# DrawingFieldEdit

Per-shape edit descriptor. The Properties panel dispatches one of

## Signature

```rust
pub enum DrawingFieldEdit
```

## Decorators

- `derive(Debug, Clone, Copy)`

## Visibility

- `pub`

## Docstring

Per-shape edit descriptor. The Properties panel dispatches one of
these per numeric text-input edit; the handler looks up the stored
drawing, applies the field change, and emits
`Command::UpdateSchDrawing` with the patched variant.
[derive(Debug, Clone, Copy)]

## Source
Lines 367–388 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
