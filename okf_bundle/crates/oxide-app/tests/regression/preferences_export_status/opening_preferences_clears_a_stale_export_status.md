---
okf_version: "0.2"
type: Function
title: opening_preferences_clears_a_stale_export_status
description: Both status lines are transient. Opening the dialog reseeds every
resource: crates/oxide-app/tests/regression/preferences_export_status.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_export_status/opening_preferences_clears_a_stale_export_status
language: rust
---

# opening_preferences_clears_a_stale_export_status

Both status lines are transient. Opening the dialog reseeds every

## Signature

```rust
fn opening_preferences_clears_a_stale_export_status()
```

## Decorators

- `test`

## Docstring

Both status lines are transient. Opening the dialog reseeds every
draft from the live value; a result from a previous session must not
come back with it and describe a write that is no longer on screen.
[test]

## Source
Lines 126–144 in `crates/oxide-app/tests/regression/preferences_export_status.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_export_status](/crates/oxide-app/tests/regression/preferences_export_status.md) |
