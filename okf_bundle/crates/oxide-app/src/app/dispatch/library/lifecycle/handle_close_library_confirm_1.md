---
okf_version: "0.2"
type: Function
title: handle_close_library_confirm
description: User picked Save All / Discard All / Cancel in the close prompt.
resource: crates/oxide-app/src/app/dispatch/library/lifecycle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/lifecycle/handle_close_library_confirm_1
language: rust
---

# handle_close_library_confirm

User picked Save All / Discard All / Cancel in the close prompt.

## Signature

```rust
pub(super) fn handle_close_library_confirm(
        &mut self,
        choice: CloseLibraryChoice,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

User picked Save All / Discard All / Cancel in the close prompt.

## Source
Lines 174–206 in `crates/oxide-app/src/app/dispatch/library/lifecycle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lifecycle](/crates/oxide-app/src/app/dispatch/library/lifecycle.md) |
