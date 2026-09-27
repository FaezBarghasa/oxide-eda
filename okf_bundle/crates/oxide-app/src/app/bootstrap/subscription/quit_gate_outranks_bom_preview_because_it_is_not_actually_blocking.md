---
okf_version: "0.2"
type: Function
title: quit_gate_outranks_bom_preview_because_it_is_not_actually_blocking
description: "#514-followup — `bom_preview_open` is NOT a `has_blocking_modal`"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/quit_gate_outranks_bom_preview_because_it_is_not_actually_blocking
language: rust
---

# quit_gate_outranks_bom_preview_because_it_is_not_actually_blocking

#514-followup — `bom_preview_open` is NOT a `has_blocking_modal`

## Signature

```rust
fn quit_gate_outranks_bom_preview_because_it_is_not_actually_blocking()
```

## Decorators

- `test`

## Docstring

#514-followup — `bom_preview_open` is NOT a `has_blocking_modal`
member (only `error_notice` / `netlist_incomplete_prompt` /
`preview` / `net_color_custom.show` are), so it must not steal Esc
from a quit/close gate the way the four true blocking modals do.
[test]

## Source
Lines 1308–1320 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
