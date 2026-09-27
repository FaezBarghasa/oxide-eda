---
okf_version: "0.2"
type: Function
title: handle_fp_library_delete_internal
description: "v0.18.8 — `Delete` button. Removes the selected"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_delete_internal
language: rust
---

# handle_fp_library_delete_internal

v0.18.8 — `Delete` button. Removes the selected

## Signature

```rust
impl Oxide { pub(in crate::app::handlers::dock::sch_library) fn handle_fp_library_delete_internal(
        &mut self,
        idx: &usize,
    ) -> bool }
```

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Docstring

v0.18.8 — `Delete` button. Removes the selected
footprint from the envelope. Refuses to remove the
last remaining footprint (an empty FootprintFile would
fail to load on next open).

## Source
Lines 77–112 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
