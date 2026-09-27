---
okf_version: "0.2"
type: Function
title: a_silent_request_never_downgrades_an_open_browser_tab_intent
description: "The reverse must not happen: a background auto-mount arriving after"
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount/a_silent_request_never_downgrades_an_open_browser_tab_intent
language: rust
---

# a_silent_request_never_downgrades_an_open_browser_tab_intent

The reverse must not happen: a background auto-mount arriving after

## Signature

```rust
fn a_silent_request_never_downgrades_an_open_browser_tab_intent()
```

## Decorators

- `test`

## Docstring

The reverse must not happen: a background auto-mount arriving after
the user asked for a tab must not silence it.
[test]

## Source
Lines 158–170 in `crates/oxide-app/tests/async_library_mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [async_library_mount](/crates/oxide-app/tests/async_library_mount.md) |
| calls | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
