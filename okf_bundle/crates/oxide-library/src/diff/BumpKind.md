---
okf_version: "0.2"
type: Class
title: BumpKind
description: "Auto-bump heuristic — `Major` when the binding shape changes (symbol,"
resource: crates/oxide-library/src/diff.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/diff/BumpKind
language: rust
---

# BumpKind

Auto-bump heuristic — `Major` when the binding shape changes (symbol,

## Signature

```rust
pub enum BumpKind
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq, Hash)`
- `non_exhaustive`

## Visibility

- `pub`

## Docstring

Auto-bump heuristic — `Major` when the binding shape changes (symbol,
footprint, sim, or pin map), `Minor` otherwise.

The variant set is preserved across the v0.9-original → v0.9-refactor-2
transition so downstream UI code that branches on `BumpKind` keeps
compiling. The version chain itself is gone (rows have no per-row
versions), but the heuristic still informs commit-message tooling and
future change-log surfaces.
[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
[non_exhaustive]

## Source
Lines 86–89 in `crates/oxide-library/src/diff.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff](/crates/oxide-library/src/diff.md) |
