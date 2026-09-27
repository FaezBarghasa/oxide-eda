---
okf_version: "0.2"
type: Function
title: placement_input_tab_cycles_rounded_rect_three_fields
description: "v0.14-footprint — Tab cycles the Rounded-Rectangle's THREE dimension"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_input_tab_cycles_rounded_rect_three_fields
language: rust
---

# placement_input_tab_cycles_rounded_rect_three_fields

v0.14-footprint — Tab cycles the Rounded-Rectangle's THREE dimension

## Signature

```rust
fn placement_input_tab_cycles_rounded_rect_three_fields()
```

## Decorators

- `test`

## Docstring

v0.14-footprint — Tab cycles the Rounded-Rectangle's THREE dimension
fields (width → height → radius → width…), parking the inactive
ones in `placement_input_others` and preserving each field's digits
across a full round-trip.
[test]

## Source
Lines 1248–1335 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
| calls | [tab](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/tab.md) |
