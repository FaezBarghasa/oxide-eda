---
okf_version: "0.2"
type: Function
title: degenerate_point_and_ordinary_arc_are_not_full_turns
description: "`start == end` is a genuine zero-sweep point-arc (raw span ~0),"
resource: crates/oxide-gfx/src/primitive/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/arc/degenerate_point_and_ordinary_arc_are_not_full_turns
language: rust
---

# degenerate_point_and_ordinary_arc_are_not_full_turns

`start == end` is a genuine zero-sweep point-arc (raw span ~0),

## Signature

```rust
fn degenerate_point_and_ordinary_arc_are_not_full_turns()
```

## Decorators

- `test`

## Docstring

`start == end` is a genuine zero-sweep point-arc (raw span ~0),
and an ordinary partial arc has a nonzero sweep — neither is a
full turn.
[test]

## Source
Lines 135–141 in `crates/oxide-gfx/src/primitive/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/primitive/arc.md) |
