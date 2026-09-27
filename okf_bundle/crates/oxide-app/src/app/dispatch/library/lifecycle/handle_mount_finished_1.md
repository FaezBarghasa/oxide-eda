---
okf_version: "0.2"
type: Function
title: handle_mount_finished
description: "A `.snxlib` mount prepared off the UI thread has landed — issue"
resource: crates/oxide-app/src/app/dispatch/library/lifecycle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/lifecycle/handle_mount_finished_1
language: rust
---

# handle_mount_finished

A `.snxlib` mount prepared off the UI thread has landed — issue

## Signature

```rust
pub(super) fn handle_mount_finished(
        &mut self,
        path: std::path::PathBuf,
        prepared: crate::library::mount::PreparedMountCell,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

A `.snxlib` mount prepared off the UI thread has landed — issue
#99 part 2c.

Order matters and both early returns are load-bearing:

1. **Intent first, from the map.** `take_mount_intent` returning
`None` means `close_library` cancelled this request while the
preparation was in flight, so the payload is dropped rather
than re-mounting a library the user just closed. Reading the
intent from the map — not from a value captured at spawn time —
is what preserves a `Silent` → `OpenBrowserTab` upgrade made by
a second request.
2. **Then the payload.** The cell is one-shot; a second read
yields `None`. iced does not clone a `Task::perform` result
today, so that should be unreachable — it warns instead of
panicking because a dropped mount is a stale cache, not a
corrupt one.

A prepare error only warns, which is exactly what both converted
call sites did synchronously. The `rfd` directory-pick path
(`handle_open_library_at`) is deliberately still synchronous and
keeps routing its errors into the recovery flow.

## Source
Lines 58–110 in `crates/oxide-app/src/app/dispatch/library/lifecycle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lifecycle](/crates/oxide-app/src/app/dispatch/library/lifecycle.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
