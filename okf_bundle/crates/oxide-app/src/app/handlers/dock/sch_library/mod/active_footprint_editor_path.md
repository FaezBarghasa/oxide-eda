---
okf_version: "0.2"
type: Function
title: active_footprint_editor_path
description: "v0.18.8 — convenience: resolve the active tab's `.snxfpt`"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/mod/active_footprint_editor_path
language: rust
---

# active_footprint_editor_path

v0.18.8 — convenience: resolve the active tab's `.snxfpt`

## Signature

```rust
impl Oxide { pub(crate) fn active_footprint_editor_path(&self) -> Option<std::path::PathBuf> }
```

## Visibility

- `pub(crate)`

## Docstring

v0.18.8 — convenience: resolve the active tab's `.snxfpt`
path, if any. The Footprint Library panel handlers below all
need this; centralising it keeps the dispatch arms tight.

## Source
Lines 46–52 in `crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_library](/crates/oxide-app/src/app/handlers/dock/sch_library/mod.md) |
