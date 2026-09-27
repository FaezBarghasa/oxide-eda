---
okf_version: "0.2"
type: Function
title: panel_card
description: Panel card / section card container with a subtle background and rounded border
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/panel_card
language: rust
---

# panel_card

Panel card / section card container with a subtle background and rounded border

## Signature

```rust
pub fn panel_card(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Panel card / section card container with a subtle background and rounded border

## Source
Lines 246–260 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_ai_diff](/crates/oxide-app/src/panels/ai_diff/view_ai_diff.md) |
| called_by | [view_copilot](/crates/oxide-app/src/panels/copilot/view_copilot.md) |
| called_by | [view_layer_stack](/crates/oxide-app/src/panels/layer_stack/view_layer_stack.md) |
| called_by | [view_mcu_console](/crates/oxide-app/src/panels/mcu_console/mod/view_mcu_console.md) |
| called_by | [view_telecom](/crates/oxide-app/src/panels/telecom/mod/view_telecom.md) |
| called_by | [view_waveform](/crates/oxide-app/src/panels/waveform/mod/view_waveform.md) |
