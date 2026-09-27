---
okf_version: "0.2"
type: Function
title: handle_recovery_library_missing_locate_result
description: "Result of the \"Locate Library\" file pick fired from the"
resource: crates/oxide-app/src/app/dispatch/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_library_missing_locate_result
language: rust
---

# handle_recovery_library_missing_locate_result

Result of the "Locate Library" file pick fired from the

## Signature

```rust
pub(super) fn handle_recovery_library_missing_locate_result(
    app: &mut Oxide,
    picked: Option<std::path::PathBuf>,
) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Result of the "Locate Library" file pick fired from the
*Library missing* recovery dialog. Clears the recovery state and,
when the user picked a replacement, re-opens the library there.

## Source
Lines 135–146 in `crates/oxide-app/src/app/dispatch/library/recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [recovery](/crates/oxide-app/src/app/dispatch/library/recovery.md) |
| called_by | [dispatch_library_message](/crates/oxide-app/src/app/dispatch/library/mod/dispatch_library_message.md) |
