---
okf_version: "0.2"
type: Function
title: direct_closed_join_leaves_status_none
description: A selection that was ALREADY a closed chain (no auto-close
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/direct_closed_join_leaves_status_none
language: rust
---

# direct_closed_join_leaves_status_none

A selection that was ALREADY a closed chain (no auto-close

## Signature

```rust
fn direct_closed_join_leaves_status_none()
```

## Decorators

- `test`

## Docstring

A selection that was ALREADY a closed chain (no auto-close
needed) leaves the status line untouched (cleared) — the
informational message is specific to the auto-close case.
[test]

## Source
Lines 304–311 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [square_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/square_editor.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
