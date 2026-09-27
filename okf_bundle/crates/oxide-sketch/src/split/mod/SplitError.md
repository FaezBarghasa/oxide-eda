---
okf_version: "0.2"
type: Class
title: SplitError
description: "Failure modes for [`split_line`]. On every variant `sketch` is left"
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/SplitError
language: rust
---

# SplitError

Failure modes for [`split_line`]. On every variant `sketch` is left

## Signature

```rust
pub enum SplitError
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, thiserror::Error)`

## Visibility

- `pub`

## Docstring

Failure modes for [`split_line`]. On every variant `sketch` is left
byte-for-byte unchanged — all validation runs against read-only
lookups before any mutation.
[derive(Clone, Copy, Debug, PartialEq, thiserror::Error)]

## Source
Lines 34–78 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
