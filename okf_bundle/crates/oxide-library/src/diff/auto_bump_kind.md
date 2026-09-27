---
okf_version: "0.2"
type: Function
title: auto_bump_kind
description: "Decide whether the change between two rows is a `Minor` or `Major`"
resource: crates/oxide-library/src/diff.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/diff/auto_bump_kind
language: rust
---

# auto_bump_kind

Decide whether the change between two rows is a `Minor` or `Major`

## Signature

```rust
pub fn auto_bump_kind(diff: &RowDiff) -> BumpKind
```

## Visibility

- `pub`

## Docstring

Decide whether the change between two rows is a `Minor` or `Major`
bump. Per the plan, any change to a primitive ref or pin-map override
is a major bump.

## Source
Lines 94–100 in `crates/oxide-library/src/diff.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff](/crates/oxide-library/src/diff.md) |
