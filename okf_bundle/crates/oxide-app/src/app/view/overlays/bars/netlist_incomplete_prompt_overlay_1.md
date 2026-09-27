---
okf_version: "0.2"
type: Function
title: netlist_incomplete_prompt_overlay
description: "#431 — netlist-incomplete \"Export anyway?\" prompt. Same modal idiom as"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/netlist_incomplete_prompt_overlay_1
language: rust
---

# netlist_incomplete_prompt_overlay

#431 — netlist-incomplete "Export anyway?" prompt. Same modal idiom as

## Signature

```rust
pub(in crate::app::view) fn netlist_incomplete_prompt_overlay(
        &self,
    ) -> Vec<Element<'_, Message>>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

#431 — netlist-incomplete "Export anyway?" prompt. Same modal idiom as
[`Self::error_notice_overlay`], but the card offers TWO actions —
"Export anyway (incomplete)" and "Cancel". Clicking outside cancels
(writes nothing). Pushes the dismiss backdrop then the prompt card.

## Source
Lines 71–81 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
