---
okf_version: "0.2"
type: Function
title: nudge_pads_translates_selection_by_delta
description: "v0.14 — \"Move Selection by X, Y…\" nudges the whole selection by one"
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests/nudge_pads_translates_selection_by_delta
language: rust
---

# nudge_pads_translates_selection_by_delta

v0.14 — "Move Selection by X, Y…" nudges the whole selection by one

## Signature

```rust
fn nudge_pads_translates_selection_by_delta()
```

## Decorators

- `test`

## Docstring

v0.14 — "Move Selection by X, Y…" nudges the whole selection by one
grid step. `nudge_pads` is the geometry the dispatcher calls; assert
it translates exactly the selected pads and leaves the rest put.
[test]

## Source
Lines 68–83 in `crates/oxide-app/src/library/editor/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/tests.md) |
