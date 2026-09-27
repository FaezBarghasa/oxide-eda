---
okf_version: "0.2"
type: Function
title: handle_prompt_library_create_options
description: "Resolution of the \"New Component Library\" save-as dialog — pops"
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration/handle_prompt_library_create_options
language: rust
---

# handle_prompt_library_create_options

Resolution of the "New Component Library" save-as dialog — pops

## Signature

```rust
impl Oxide { pub(super) fn handle_prompt_library_create_options(
        &mut self,
        project_path: std::path::PathBuf,
        lib_path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Resolution of the "New Component Library" save-as dialog — pops
the "Library Options" modal (Git / LFS opt-in) rather than
creating immediately.

## Source
Lines 15–34 in `crates/oxide-app/src/app/dispatch/library/registration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [registration](/crates/oxide-app/src/app/dispatch/library/registration.md) |
