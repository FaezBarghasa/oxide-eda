---
okf_version: "0.2"
type: Function
title: create_options_overlay
description: "\"Library Options\" modal — pops between the New Library Save-As"
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/create_options_overlay
language: rust
---

# create_options_overlay

"Library Options" modal — pops between the New Library Save-As

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn create_options_overlay(&self) -> Option<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

"Library Options" modal — pops between the New Library Save-As
dialog and the actual `LocalGitAdapter::init` so the user can opt
into Git LFS for binary 3D models before anything hits disk.

## Source
Lines 401–418 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
