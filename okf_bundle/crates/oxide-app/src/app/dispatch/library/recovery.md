---
okf_version: "0.2"
type: Module
title: recovery
description: "Open-error recovery plumbing — the shared atomic-write helper,"
resource: crates/oxide-app/src/app/dispatch/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/recovery
language: rust
---

# recovery

Open-error recovery plumbing — the shared atomic-write helper,

## Docstring

Open-error recovery plumbing — the shared atomic-write helper,
`route_open_error` classification, and the per-choice recovery
actions (library-missing / git-missing / broken-binding).

Extracted verbatim from the library dispatcher (`dispatch/library`);
pure code motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
| related | [route_open_error](/crates/oxide-app/src/app/dispatch/library/recovery/route_open_error.md) |
| related | [handle_recovery_library_missing](/crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_library_missing.md) |
| related | [handle_recovery_library_missing_locate_result](/crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_library_missing_locate_result.md) |
| related | [handle_recovery_git_missing](/crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_git_missing.md) |
| related | [handle_recovery_broken_binding](/crates/oxide-app/src/app/dispatch/library/recovery/handle_recovery_broken_binding.md) |
