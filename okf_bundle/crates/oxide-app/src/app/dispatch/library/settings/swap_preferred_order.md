---
okf_version: "0.2"
type: Function
title: swap_preferred_order
description: "Move `src` one slot up (or down) the preferred-order list and"
resource: crates/oxide-app/src/app/dispatch/library/settings.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/settings/swap_preferred_order
language: rust
---

# swap_preferred_order

Move `src` one slot up (or down) the preferred-order list and

## Signature

```rust
impl Oxide { fn swap_preferred_order(&mut self, src: oxide_library::DistributorSource, up: bool) }
```

## Docstring

Move `src` one slot up (or down) the preferred-order list and
persist. A save failure lands in `preferred_order_error` so the
panel can show it inline — silently swallowing it left the user
to discover the reverted order on next launch.

## Source
Lines 204–223 in `crates/oxide-app/src/app/dispatch/library/settings.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [settings](/crates/oxide-app/src/app/dispatch/library/settings.md) |
| calls | [save_preferred_order](/crates/oxide-app/src/library/settings/persistence/save_preferred_order.md) |
