---
okf_version: "0.2"
type: Function
title: handle_fp_library_add_internal
description: "v0.18.8 — `+ Add` button. Routes through the existing"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_add_internal_1
language: rust
---

# handle_fp_library_add_internal

v0.18.8 — `+ Add` button. Routes through the existing

## Signature

```rust
pub(in crate::app::handlers::dock::sch_library) fn handle_fp_library_add_internal(
        &mut self,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Docstring

v0.18.8 — `+ Add` button. Routes through the existing
`FootprintAddNewSibling` dispatcher which appends an
empty Footprint and switches `active_idx` onto it.

## Source
Lines 49–71 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.md) |
