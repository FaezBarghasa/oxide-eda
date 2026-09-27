---
okf_version: "0.2"
type: Function
title: save_all_failure_listing
description: "Format the per-file failure lines (`name — reason`) for a"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/save_all_failure_listing_1
language: rust
---

# save_all_failure_listing

Format the per-file failure lines (`name — reason`) for a

## Signature

```rust
fn save_all_failure_listing(failed: &[(std::path::PathBuf, String)]) -> String
```

## Docstring

Format the per-file failure lines (`name — reason`) for a
Save-All error modal so the user sees *why* each file couldn't
be saved, not just its name.

## Source
Lines 196–208 in `crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [close_project](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.md) |
