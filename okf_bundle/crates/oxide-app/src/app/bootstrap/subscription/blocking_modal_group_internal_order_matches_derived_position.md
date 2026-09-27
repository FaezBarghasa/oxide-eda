---
okf_version: "0.2"
type: Function
title: blocking_modal_group_internal_order_matches_derived_position
description: "Pin the `has_blocking_modal` group's own internal order, derived"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/blocking_modal_group_internal_order_matches_derived_position
language: rust
---

# blocking_modal_group_internal_order_matches_derived_position

Pin the `has_blocking_modal` group's own internal order, derived

## Signature

```rust
fn blocking_modal_group_internal_order_matches_derived_position()
```

## Decorators

- `test`

## Docstring

Pin the `has_blocking_modal` group's own internal order, derived
from `collect_overlays:750-754` reversed: net-colour > bom-preview >
print-preview > netlist-incomplete-prompt > export-error.
`bom_preview_open` sits inside this chain even though it isn't a
`has_blocking_modal` member — it paints in the same early block, one
slot below `net_color_custom_open` and one above `print_preview_open`.
[test]

## Source
Lines 1172–1209 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
