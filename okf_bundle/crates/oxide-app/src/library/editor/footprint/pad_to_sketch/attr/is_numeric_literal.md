---
okf_version: "0.2"
type: Function
title: is_numeric_literal
description: "True when `expr` is a plain number with an optional unit suffix"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/is_numeric_literal
language: rust
---

# is_numeric_literal

True when `expr` is a plain number with an optional unit suffix

## Signature

```rust
fn is_numeric_literal(expr: &str) -> bool
```

## Docstring

True when `expr` is a plain number with an optional unit suffix
(`90`, `-45.5deg`, `1rad`, and the `=`-prefixed forms of each) —
i.e. carries no parameter reference or arithmetic, so overwriting
it loses nothing the user authored by name.

## Source
Lines 97–103 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.md) |
