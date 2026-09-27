---
okf_version: "0.2"
type: Function
title: unmapped_command_ids_only_shrink
description: "Coverage ratchet: the set of catalog ids with no dispatch arm may"
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/unmapped_command_ids_only_shrink
language: rust
---

# unmapped_command_ids_only_shrink

Coverage ratchet: the set of catalog ids with no dispatch arm may

## Signature

```rust
fn unmapped_command_ids_only_shrink()
```

## Decorators

- `test`

## Docstring

Coverage ratchet: the set of catalog ids with no dispatch arm may
only shrink.

The bridge's `_ => return None` fallthrough makes an unmapped id
indistinguishable from a mapped one at the call site — the keymap
resolves it, consumes the stroke, and no-ops. Nothing measured that
gap before this test, so it grew to 75 of 134 unnoticed before anything measured it.

Deliberately two assertions rather than one set equality: the
"no longer unmapped" direction is a fix and gets its own message
telling you to update the list, while the "newly unmapped"
direction is the regression this exists to catch.
[test]

## Source
Lines 376–404 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [resolvable_command_ids](/crates/oxide-app/src/app/command/bridge/resolvable_command_ids.md) |
| calls | [all_command_ids](/crates/oxide-app/src/keymap/catalog/mod/all_command_ids.md) |
