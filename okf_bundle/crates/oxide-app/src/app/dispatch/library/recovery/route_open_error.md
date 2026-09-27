---
okf_version: "0.2"
type: Function
title: route_open_error
description: "Classify a `LocalGitAdapter::open` error and, if recoverable,"
resource: crates/oxide-app/src/app/dispatch/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/recovery/route_open_error
language: rust
---

# route_open_error

Classify a `LocalGitAdapter::open` error and, if recoverable,

## Signature

```rust
pub(crate) fn route_open_error(
    state: &mut LibraryState,
    path: &std::path::Path,
    err: &LibraryError,
)
```

## Visibility

- `pub(crate)`

## Docstring

Classify a `LocalGitAdapter::open` error and, if recoverable,
stash the matching `RecoveryDialog` on `LibraryState::recovery`.
Unrecoverable errors are left alone — the caller's `tracing::warn!`
is the only surface.

String-matches the error message produced by `LocalGitAdapter::open`
because the underlying `LibraryError` enum doesn't carry structured
"missing-snxlib" / "missing-git" variants in v0.9. This is the
lower-effort path called out in `v0.9-snxlib-as-file-plan.md` §2
Stage H — adding `LibraryError::MissingGitRepo` /
`LibraryError::MissingSnxlibFile` variants is a clean follow-up
once the rest of v0.9 settles.

## Source
Lines 48–78 in `crates/oxide-app/src/app/dispatch/library/recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [recovery](/crates/oxide-app/src/app/dispatch/library/recovery.md) |
| called_by | [handle_open_library_at](/crates/oxide-app/src/app/dispatch/library/lifecycle/handle_open_library_at.md) |
