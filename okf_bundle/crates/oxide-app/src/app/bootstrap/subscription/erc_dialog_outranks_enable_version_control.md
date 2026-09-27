---
okf_version: "0.2"
type: Function
title: erc_dialog_outranks_enable_version_control
description: "Round-2 regression #1: `enable_vc_open` was hand-placed above"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/erc_dialog_outranks_enable_version_control
language: rust
---

# erc_dialog_outranks_enable_version_control

Round-2 regression #1: `enable_vc_open` was hand-placed above

## Signature

```rust
fn erc_dialog_outranks_enable_version_control()
```

## Decorators

- `test`

## Docstring

Round-2 regression #1: `enable_vc_open` was hand-placed above
`erc_open`, but `erc_dialog_open` paints at `view/mod.rs:792`
(`detachable_dialogs_overlay`, pushed last inside it), strictly
later than `enable_vc_open`'s home in `simple_dialogs_overlay`
(`:791`) — so ERC must outrank Enable Version Control.
[test]

## Source
Lines 1217–1227 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
