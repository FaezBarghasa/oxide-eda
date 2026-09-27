---
okf_version: "0.2"
type: Function
title: grid_properties_outranks_enable_version_control
description: "Round-2 regression #2: `enable_vc_open` was also hand-placed above"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/grid_properties_outranks_enable_version_control
language: rust
---

# grid_properties_outranks_enable_version_control

Round-2 regression #2: `enable_vc_open` was also hand-placed above

## Signature

```rust
fn grid_properties_outranks_enable_version_control()
```

## Decorators

- `test`

## Docstring

Round-2 regression #2: `enable_vc_open` was also hand-placed above
`grid_properties_open`, but inside `simple_dialogs_overlay`
(`modals.rs:118-141`) `grid_properties` is pushed AFTER `enable_vc`
(`:133` then `:136`) — later push = paints on top — so Grid
Properties must outrank Enable Version Control, not the reverse.
[test]

## Source
Lines 1235–1245 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
