---
okf_version: "0.2"
type: Function
title: empty
description: Empty symbol scaffold used by New Symbol flows.
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/empty_1
language: rust
---

# empty

Empty symbol scaffold used by New Symbol flows.

## Signature

```rust
pub fn empty(name: impl Into<String>) -> Self
```

## Visibility

- `pub`

## Docstring

Empty symbol scaffold used by New Symbol flows.

## Source
Lines 406–429 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
| calls | [default_designator](/crates/oxide-library/src/primitive/symbol/mod/default_designator.md) |
| calls | [default_comment](/crates/oxide-library/src/primitive/symbol/mod/default_comment.md) |
| calls | [default_version](/crates/oxide-library/src/primitive/symbol/mod/default_version.md) |
