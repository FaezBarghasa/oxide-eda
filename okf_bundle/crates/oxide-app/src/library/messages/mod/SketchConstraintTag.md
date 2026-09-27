---
okf_version: "0.2"
type: Class
title: SketchConstraintTag
description: v0.13.3 — selection-aware constraint kind tag. The dispatcher
resource: crates/oxide-app/src/library/messages/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/mod/SketchConstraintTag
language: rust
---

# SketchConstraintTag

v0.13.3 — selection-aware constraint kind tag. The dispatcher

## Signature

```rust
pub enum SketchConstraintTag
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

v0.13.3 — selection-aware constraint kind tag. The dispatcher
resolves these against the editor's primary + secondary
selection slots into the matching `ConstraintKind` and emits the
SketchEdit. A tag that doesn't apply to the current selection, or
whose dimension field cannot be read as a number, adds no constraint
and is reported to the Messages panel and the sketch warning list
(#599) — it is never a silent no-op.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 313–355 in `crates/oxide-app/src/library/messages/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/messages/mod.md) |
