---
okf_version: "0.2"
type: Class
title: CommandGroup
description: Coarse editor-surface bucket used by the Keyboard Shortcuts pane to
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/CommandGroup
language: rust
---

# CommandGroup

Coarse editor-surface bucket used by the Keyboard Shortcuts pane to

## Signature

```rust
pub enum CommandGroup
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Coarse editor-surface bucket used by the Keyboard Shortcuts pane to
group commands for display. Distinct from [`CommandMetadata::category`],
which stays fine-grained (place / edit / view …); the group is the
primary *surface* a command belongs to.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 14–25 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
