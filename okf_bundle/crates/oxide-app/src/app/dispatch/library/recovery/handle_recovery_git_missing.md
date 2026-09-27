---
okf_version: "0.2"
type: Function
title: handle_recovery_git_missing
description: "Handle the user's choice from the *Git missing* recovery dialog."
resource: crates/oxide-app/src/app/dispatch/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_git_missing
language: rust
---

# handle_recovery_git_missing

Handle the user's choice from the *Git missing* recovery dialog.

## Signature

```rust
pub(super) fn handle_recovery_git_missing(
    app: &mut Oxide,
    choice: GitMissingChoice,
) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Handle the user's choice from the *Git missing* recovery dialog.

## Source
Lines 149–187 in `crates/oxide-app/src/app/dispatch/library/recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [recovery](/crates/oxide-app/src/app/dispatch/library/recovery.md) |
| called_by | [dispatch_library_message](/crates/oxide-app/src/app/dispatch/library/mod/dispatch_library_message.md) |
