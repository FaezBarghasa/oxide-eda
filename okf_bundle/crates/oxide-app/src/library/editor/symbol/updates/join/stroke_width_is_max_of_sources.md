---
okf_version: "0.2"
type: Function
title: stroke_width_is_max_of_sources
description: "The joined polygon's stroke width is the max of the source"
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/stroke_width_is_max_of_sources
language: rust
---

# stroke_width_is_max_of_sources

The joined polygon's stroke width is the max of the source

## Signature

```rust
fn stroke_width_is_max_of_sources()
```

## Decorators

- `test`

## Docstring

The joined polygon's stroke width is the max of the source
graphics' stroke widths.
[test]

## Source
Lines 412–449 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
