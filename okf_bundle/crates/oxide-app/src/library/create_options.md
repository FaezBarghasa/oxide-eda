---
okf_version: "0.2"
type: Module
title: create_options
description: "\"Library Options\" modal — Stage 11 of"
resource: crates/oxide-app/src/library/create_options.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/create_options
language: rust
---

# create_options

"Library Options" modal — Stage 11 of

## Docstring

"Library Options" modal — Stage 11 of
`v0.9-snxlib-as-file-plan.md`.

Pops up between the New Library Save-As dialog (where the user
picked the `.snxlib` filename) and the actual disk + git init
call. Carries a single user-facing toggle: "Use Git LFS for
binary 3D models". Defaults off so `git lfs` doesn't become a
hard prerequisite for casual library creation; production
libraries flip it on at create time so 3D model commits don't
bloat the git pack.

Mounted as a full-screen overlay backdrop via
`app/view/mod.rs::collect_overlays` when
`LibraryState::create_options.is_some()`. The matching
`needs_overlay` predicate must include the same flag (memory
note `[needs_overlay predicate gates modal rendering]`).

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/create_options/view.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
