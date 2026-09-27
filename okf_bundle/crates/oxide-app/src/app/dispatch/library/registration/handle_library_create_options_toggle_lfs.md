---
okf_version: "0.2"
type: Function
title: handle_library_create_options_toggle_lfs
description: "\"Library Options\" modal — toggle the \"Use Git LFS\" checkbox."
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration/handle_library_create_options_toggle_lfs
language: rust
---

# handle_library_create_options_toggle_lfs

"Library Options" modal — toggle the "Use Git LFS" checkbox.

## Signature

```rust
impl Oxide { pub(super) fn handle_library_create_options_toggle_lfs(&mut self) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

"Library Options" modal — toggle the "Use Git LFS" checkbox.

## Source
Lines 37–42 in `crates/oxide-app/src/app/dispatch/library/registration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [registration](/crates/oxide-app/src/app/dispatch/library/registration.md) |
