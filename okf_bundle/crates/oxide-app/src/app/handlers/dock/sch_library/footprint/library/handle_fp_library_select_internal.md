---
okf_version: "0.2"
type: Function
title: handle_fp_library_select_internal
description: v0.18.8 — Footprint Library panel internal-row select.
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_select_internal
language: rust
---

# handle_fp_library_select_internal

v0.18.8 — Footprint Library panel internal-row select.

## Signature

```rust
impl Oxide { pub(in crate::app::handlers::dock::sch_library) fn handle_fp_library_select_internal(
        &mut self,
        idx: &usize,
    ) -> bool }
```

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Docstring

v0.18.8 — Footprint Library panel internal-row select.
Stores `panel_selected_idx` on the active footprint
editor so the row tints + button row gates correctly.
Independent of `active_idx`: only the Edit button (or
a double-click hook later) promotes selection to active.

## Source
Lines 32–44 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.md) |
