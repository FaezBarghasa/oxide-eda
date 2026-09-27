---
okf_version: "0.2"
type: Function
title: dispatch_library_message
resource: crates/oxide-app/src/app/dispatch/library/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/mod/dispatch_library_message
language: rust
---

# dispatch_library_message

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_library_message(&mut self, msg: LibraryMessage) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 56–409 in `crates/oxide-app/src/app/dispatch/library/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/app/dispatch/library/mod.md) |
| calls | [jump_to_use_site](/crates/oxide-app/src/library/commands/jump_to_use_site.md) |
| calls | [handle_recovery_library_missing](/crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_library_missing.md) |
| calls | [handle_recovery_library_missing_locate_result](/crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_library_missing_locate_result.md) |
| calls | [handle_recovery_git_missing](/crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_git_missing.md) |
| calls | [handle_recovery_broken_binding](/crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_broken_binding.md) |
