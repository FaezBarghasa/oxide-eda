---
okf_version: "0.2"
type: Class
title: SplitResult
description: "Result of a successful [`split_line`] — the new mid `Point` and the"
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/SplitResult
language: rust
---

# SplitResult

Result of a successful [`split_line`] — the new mid `Point` and the

## Signature

```rust
pub struct SplitResult
```

## Decorators

- `derive(Clone, Debug, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Result of a successful [`split_line`] — the new mid `Point` and the
two replacement `Line`s (`start -> mid`, `mid -> end`), so a caller
can select or further constrain them without re-querying the
sketch.
[derive(Clone, Debug, PartialEq, Eq)]

## Methods

- `mid_point`
- `line_a`
- `line_b`
- `dropped_constraints`

## Source
Lines 85–98 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
