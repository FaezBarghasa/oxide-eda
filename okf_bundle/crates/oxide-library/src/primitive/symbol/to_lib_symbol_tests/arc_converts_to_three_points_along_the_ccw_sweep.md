---
okf_version: "0.2"
type: Function
title: arc_converts_to_three_points_along_the_ccw_sweep
description: "`SymbolGraphicKind::Arc` (`center`/`radius`/CCW `start_deg..end_deg`)"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/arc_converts_to_three_points_along_the_ccw_sweep
language: rust
---

# arc_converts_to_three_points_along_the_ccw_sweep

`SymbolGraphicKind::Arc` (`center`/`radius`/CCW `start_deg..end_deg`)

## Signature

```rust
fn arc_converts_to_three_points_along_the_ccw_sweep()
```

## Decorators

- `test`

## Docstring

`SymbolGraphicKind::Arc` (`center`/`radius`/CCW `start_deg..end_deg`)
converts into `Graphic::Arc`'s three-point (`start`/`mid`/`end`)
representation along the same CCW sweep.
[test]

## Source
Lines 316–344 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol_tests](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.md) |
| calls | [approx](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/approx.md) |
