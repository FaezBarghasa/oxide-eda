---
okf_version: "0.2"
type: Function
title: floating
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/floating
language: rust
---

# floating

## Signature

```rust
fn floating(kind: PanelKind) -> FloatingPanel
```

## Source
Lines 342–351 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
| called_by | [a_floating_panel_blocks_a_second_docked_copy](/crates/oxide-app/src/dock/state/a_floating_panel_blocks_a_second_docked_copy.md) |
| called_by | [dropping_a_floating_panel_on_a_zone_docks_it_there](/crates/oxide-app/src/dock/state/dropping_a_floating_panel_on_a_zone_docks_it_there.md) |
| called_by | [re_docking_a_floating_panel_sends_it_to_its_home_region](/crates/oxide-app/src/dock/state/re_docking_a_floating_panel_sends_it_to_its_home_region.md) |
