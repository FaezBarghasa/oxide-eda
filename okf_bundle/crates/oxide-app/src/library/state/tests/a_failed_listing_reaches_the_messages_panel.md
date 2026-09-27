---
okf_version: "0.2"
type: Function
title: a_failed_listing_reaches_the_messages_panel
description: "The keep-the-cache behaviour is only half the fix: silently keeping"
resource: crates/oxide-app/src/library/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/state/tests/a_failed_listing_reaches_the_messages_panel
language: rust
---

# a_failed_listing_reaches_the_messages_panel

The keep-the-cache behaviour is only half the fix: silently keeping

## Signature

```rust
fn a_failed_listing_reaches_the_messages_panel()
```

## Decorators

- `test`

## Docstring

The keep-the-cache behaviour is only half the fix: silently keeping
a stale cache is still a swallowed error, so the failure has to
reach the Messages panel.
[test]

## Source
Lines 297–324 in `crates/oxide-app/src/library/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/state/tests.md) |
| calls | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
| calls | [library_with_cached_primitives](/crates/oxide-app/src/library/state/tests/library_with_cached_primitives.md) |
| calls | [recent_entries](/crates/oxide-app/src/diagnostics/recent_entries.md) |
