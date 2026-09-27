---
okf_version: "0.2"
type: Function
title: handle_library_create_options_toggle_git
description: "\"Library Options\" modal — toggle the \"Enable version control\""
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration/handle_library_create_options_toggle_git
language: rust
---

# handle_library_create_options_toggle_git

"Library Options" modal — toggle the "Enable version control"

## Signature

```rust
impl Oxide { pub(super) fn handle_library_create_options_toggle_git(&mut self) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

"Library Options" modal — toggle the "Enable version control"
checkbox.

## Source
Lines 46–58 in `crates/oxide-app/src/app/dispatch/library/registration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [registration](/crates/oxide-app/src/app/dispatch/library/registration.md) |
