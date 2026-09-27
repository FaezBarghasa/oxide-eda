---
okf_version: "0.2"
type: Function
title: handle_open_library_at
description: "Result of the `rfd` directory pick — mount the library at `path`,"
resource: crates/oxide-app/src/app/dispatch/library/lifecycle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/lifecycle/handle_open_library_at_1
language: rust
---

# handle_open_library_at

Result of the `rfd` directory pick — mount the library at `path`,

## Signature

```rust
pub(super) fn handle_open_library_at(&mut self, path: std::path::PathBuf) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Result of the `rfd` directory pick — mount the library at `path`,
routing any open error into the recovery flow.

## Source
Lines 28–34 in `crates/oxide-app/src/app/dispatch/library/lifecycle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lifecycle](/crates/oxide-app/src/app/dispatch/library/lifecycle.md) |
| calls | [route_open_error](/crates/oxide-app/src/app/dispatch/library/recovery/route_open_error.md) |
