---
okf_version: "0.2"
type: Function
title: move_command_palette_selection
resource: crates/oxide-app/src/app/dispatch/command_palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/command_palette/move_command_palette_selection
language: rust
---

# move_command_palette_selection

## Signature

```rust
impl Oxide { fn move_command_palette_selection(&mut self, delta: i32) }
```

## Source
Lines 53–69 in `crates/oxide-app/src/app/dispatch/command_palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_palette](/crates/oxide-app/src/app/dispatch/command_palette.md) |
| calls | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
| calls | [rank_results](/crates/oxide-app/src/app/command_palette/rank_results.md) |
