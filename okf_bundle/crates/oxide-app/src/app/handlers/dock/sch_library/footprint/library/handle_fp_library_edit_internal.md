---
okf_version: "0.2"
type: Function
title: handle_fp_library_edit_internal
description: "v0.18.8 — `Edit` button. Promotes the panel selection"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_edit_internal
language: rust
---

# handle_fp_library_edit_internal

v0.18.8 — `Edit` button. Promotes the panel selection

## Signature

```rust
impl Oxide { pub(in crate::app::handlers::dock::sch_library) fn handle_fp_library_edit_internal(
        &mut self,
        idx: &usize,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Docstring

v0.18.8 — `Edit` button. Promotes the panel selection
to `active_idx` via the existing
`FootprintSelectActiveIdx` dispatcher.

## Source
Lines 117–134 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.md) |
