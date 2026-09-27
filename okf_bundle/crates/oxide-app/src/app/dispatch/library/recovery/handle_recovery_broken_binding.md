---
okf_version: "0.2"
type: Function
title: handle_recovery_broken_binding
description: "Handle the user's choice from the *Broken primitive binding* dialog."
resource: crates/oxide-app/src/app/dispatch/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_broken_binding
language: rust
---

# handle_recovery_broken_binding

Handle the user's choice from the *Broken primitive binding* dialog.

## Signature

```rust
pub(super) fn handle_recovery_broken_binding(
    app: &mut Oxide,
    _choice: BrokenBindingChoice,
) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Handle the user's choice from the *Broken primitive binding* dialog.

v0.9 stub: the dispatch path that detects broken bindings hasn't
landed yet (Stage 12+ wires the row-load checks). The handler
therefore only knows how to close the dialog; the actual rebind /
remove-row flows queue behind the detection plumbing. The dialog
surface itself ships now so the overlay layer is in place.

## Source
Lines 196–202 in `crates/oxide-app/src/app/dispatch/library/recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [recovery](/crates/oxide-app/src/app/dispatch/library/recovery.md) |
| called_by | [dispatch_library_message](/crates/oxide-app/src/app/dispatch/library/mod/dispatch_library_message.md) |
