---
okf_version: "0.2"
type: Function
title: handle_fp_library_open_sibling
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_open_sibling
language: rust
---

# handle_fp_library_open_sibling

## Signature

```rust
impl Oxide { pub(in crate::app::handlers::dock::sch_library) fn handle_fp_library_open_sibling(
        &mut self,
        sibling_path: &std::path::Path,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Source
Lines 15–25 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.md) |
