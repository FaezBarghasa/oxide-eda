---
okf_version: "0.2"
type: Function
title: handle_fp_editor_set_pad_stack_tab
description: v0.20/v0.21 — Pad Stack tab + placement-default Net / Locked /
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_pad_stack_tab_1
language: rust
---

# handle_fp_editor_set_pad_stack_tab

v0.20/v0.21 — Pad Stack tab + placement-default Net / Locked /

## Signature

```rust
pub(in crate::app::handlers::dock::sch_library) fn handle_fp_editor_set_pad_stack_tab(
        &mut self,
        tab: &crate::library::editor::footprint::state::PadStackTab,
    ) -> bool
```

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Docstring

v0.20/v0.21 — Pad Stack tab + placement-default Net / Locked /
Electrical Type / Hole detail / Plated toggles. Each mutates a
slice of `editor.state.next_pad_defaults` (or `pad_stack_tab`)
so the next placement click picks up the new value; the panel
refresh re-reads the form.

## Source
Lines 657–666 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
