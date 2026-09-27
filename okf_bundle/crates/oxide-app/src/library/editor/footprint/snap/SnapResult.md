---
okf_version: "0.2"
type: Class
title: SnapResult
description: "Outcome of a cursor snap. The `pos` field is the canvas's working"
resource: crates/oxide-app/src/library/editor/footprint/snap.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/snap/SnapResult
language: rust
---

# SnapResult

Outcome of a cursor snap. The `pos` field is the canvas's working

## Signature

```rust
pub struct SnapResult
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

Outcome of a cursor snap. The `pos` field is the canvas's working
world coordinate after snapping; the `kind` discriminates which
snap policy fired so the canvas can render an indicator badge.
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `pos`
- `kind`

## Source
Lines 47–50 in `crates/oxide-app/src/library/editor/footprint/snap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap](/crates/oxide-app/src/library/editor/footprint/snap.md) |
