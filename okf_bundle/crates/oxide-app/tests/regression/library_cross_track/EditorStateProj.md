---
okf_version: "0.2"
type: Class
title: EditorStateProj
description: "Phase-5 helper — small projection of the editor's persisted state"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/EditorStateProj
language: rust
---

# EditorStateProj

Phase-5 helper — small projection of the editor's persisted state

## Signature

```rust
struct EditorStateProj
```

## Decorators

- `derive(Debug, Clone, Eq, PartialEq)`

## Docstring

Phase-5 helper — small projection of the editor's persisted state
for before/after snapshots. Captures sketch entity / constraint /
parameter counts and pad count; sufficient to detect that an
undo/redo cycle reverted the placement (and not just paged a flag).
[derive(Debug, Clone, Eq, PartialEq)]

## Methods

- `pads`
- `entities`
- `constraints`
- `parameters`

## Source
Lines 86–91 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
