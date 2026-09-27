---
okf_version: "0.2"
type: Class
title: BoxSelectKind
description: Box selection mode — determined by drag direction.
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/BoxSelectKind
language: rust
---

# BoxSelectKind

Box selection mode — determined by drag direction.

## Signature

```rust
pub enum BoxSelectKind
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Box selection mode — determined by drag direction.

`Window` selects items **fully contained** within the box
(left-to-right drag, blue outline).
`Crossing` selects items that **touch or intersect** the box
(right-to-left drag, green outline).
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 82–85 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
