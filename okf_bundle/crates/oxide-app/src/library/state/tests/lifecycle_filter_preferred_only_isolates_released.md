---
okf_version: "0.2"
type: Function
title: lifecycle_filter_preferred_only_isolates_released
description: "LifecycleFilter Released-only mode keeps `Released` rows and"
resource: crates/oxide-app/src/library/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/state/tests/lifecycle_filter_preferred_only_isolates_released
language: rust
---

# lifecycle_filter_preferred_only_isolates_released

LifecycleFilter Released-only mode keeps `Released` rows and

## Signature

```rust
fn lifecycle_filter_preferred_only_isolates_released()
```

## Decorators

- `test`

## Docstring

LifecycleFilter Released-only mode keeps `Released` rows and
drops every other state — guards plan §6's "preferred only"
pivot from drifting back to "active + preferred".
[test]

## Source
Lines 120–128 in `crates/oxide-app/src/library/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/state/tests.md) |
