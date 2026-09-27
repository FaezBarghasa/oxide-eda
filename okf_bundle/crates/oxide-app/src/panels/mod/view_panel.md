---
okf_version: "0.2"
type: Function
title: view_panel
description: "Render a panel's content."
resource: crates/oxide-app/src/panels/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:31:39Z"
concept_id: crates/oxide-app/src/panels/mod/view_panel
language: rust
---

# view_panel

Render a panel's content.

## Signature

```rust
pub fn view_panel(kind: PanelKind, ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render a panel's content.

## Source
Lines 253–307 in `crates/oxide-app/src/panels/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [panels](/crates/oxide-app/src/panels/mod.md) |
| calls | [view_components](/crates/oxide-app/src/panels/components/view_components.md) |
| calls | [view_waveform](/crates/oxide-app/src/panels/waveform/mod/view_waveform.md) |
| calls | [view_telecom](/crates/oxide-app/src/panels/telecom/mod/view_telecom.md) |
| calls | [view_mcu_console](/crates/oxide-app/src/panels/mcu_console/mod/view_mcu_console.md) |
| calls | [view_projects](/crates/oxide-app/src/panels/projects/view_projects.md) |
| calls | [view_navigator](/crates/oxide-app/src/panels/projects/view_navigator.md) |
| calls | [view_properties](/crates/oxide-app/src/panels/properties/view_properties.md) |
| calls | [view_stub](/crates/oxide-app/src/panels/widgets/view_stub.md) |
| calls | [view_erc](/crates/oxide-app/src/panels/status/view_erc.md) |
| calls | [view_messages](/crates/oxide-app/src/panels/status/view_messages.md) |
| calls | [view_drc](/crates/oxide-app/src/panels/drc/view_drc.md) |
| calls | [view_layer_stack](/crates/oxide-app/src/panels/layer_stack/view_layer_stack.md) |
| calls | [view_copilot](/crates/oxide-app/src/panels/copilot/view_copilot.md) |
| calls | [view_ai_diff](/crates/oxide-app/src/panels/ai_diff/view_ai_diff.md) |
| calls | [view_sch_library](/crates/oxide-app/src/panels/library/view_sch_library.md) |
| calls | [view_footprint_library](/crates/oxide-app/src/panels/library/view_footprint_library.md) |
| calls | [view_history](/crates/oxide-app/src/panels/history/view_history.md) |
| called_by | [view](/crates/oxide-app/src/app/view/mod/view.md) |
| called_by | [view_floating_panel](/crates/oxide-app/src/dock/view/view_floating_panel.md) |
| called_by | [view_region](/crates/oxide-app/src/dock/view/view_region.md) |
