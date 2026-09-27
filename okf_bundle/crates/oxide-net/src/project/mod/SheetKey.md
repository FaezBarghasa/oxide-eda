---
okf_version: "0.2"
type: Class
title: SheetKey
description: "Opaque, host-neutral identifier for one sheet within a [`ProjectGraph`] —"
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/SheetKey
language: rust
---

# SheetKey

Opaque, host-neutral identifier for one sheet within a [`ProjectGraph`] —

## Signature

```rust
pub struct SheetKey
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)`

## Visibility

- `pub`

## Docstring

Opaque, host-neutral identifier for one sheet within a [`ProjectGraph`] —
a resolved path made relative to a fixed base and normalized by the
caller. This crate never interprets a `SheetKey` as a path: no joining,
normalizing, or case-folding happens here, only comparison.

A newtype, not an alias, on purpose. The whole point of #466 is that a
resolved key and a raw `ChildSheet.filename` reference string are
different things that happen to share a representation — they sit side by
side in [`ProjectGraph::resolved`], and under an alias every one of those
call sites would still compile with the two swapped, silently resolving
the wrong sheet. That is the defect this module exists to remove, so the
distinction is carried in the type system rather than in a comment.
[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]

## Source
Lines 66–66 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
