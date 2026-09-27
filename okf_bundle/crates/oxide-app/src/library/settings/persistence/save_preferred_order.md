---
okf_version: "0.2"
type: Function
title: save_preferred_order
description: "Persist the preferred-order list. Errors warn through `tracing` and"
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/save_preferred_order
language: rust
---

# save_preferred_order

Persist the preferred-order list. Errors warn through `tracing` and

## Signature

```rust
pub fn save_preferred_order(order: &[DistributorSource]) -> Result<(), String>
```

## Visibility

- `pub`

## Docstring

Persist the preferred-order list. Errors warn through `tracing` and
surface to the caller via the `Result` so the dispatcher can show a
brief inline error in the Distributor APIs settings panel — mirrors
`components_panel::global_prefs::save`. Swallowing this made a failed
save invisible until the user found the setting reverted next launch.

## Source
Lines 158–167 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| calls | [config_path](/crates/oxide-app/src/library/settings/persistence/config_path.md) |
| calls | [save_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/save_preferred_order_at.md) |
| called_by | [swap_preferred_order](/crates/oxide-app/src/app/dispatch/library/settings/swap_preferred_order.md) |
