---
okf_version: "0.2"
type: Function
title: every_palette_command_row_resolves_through_the_bridge
description: "#366 — every command row the palette offers must resolve through"
resource: crates/oxide-app/src/app/command_palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command_palette/every_palette_command_row_resolves_through_the_bridge
language: rust
---

# every_palette_command_row_resolves_through_the_bridge

#366 — every command row the palette offers must resolve through

## Signature

```rust
fn every_palette_command_row_resolves_through_the_bridge()
```

## Decorators

- `test`

## Docstring

#366 — every command row the palette offers must resolve through
the bridge. A row that dispatches nothing looks identical to one
that works, so this is the only thing standing between the user
and a menu of silent no-ops.
[test]

## Source
Lines 354–377 in `crates/oxide-app/src/app/command_palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_palette](/crates/oxide-app/src/app/command_palette.md) |
| calls | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
| calls | [is_dispatchable](/crates/oxide-app/src/app/command/bridge/is_dispatchable.md) |
