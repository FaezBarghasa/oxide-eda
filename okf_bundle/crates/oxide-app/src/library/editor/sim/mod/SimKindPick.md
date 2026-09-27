---
okf_version: "0.2"
type: Class
title: SimKindPick
description: "Pick-list adapter so `SimKind` can sit on the iced `pick_list`"
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod/SimKindPick
language: rust
---

# SimKindPick

Pick-list adapter so `SimKind` can sit on the iced `pick_list`

## Signature

```rust
struct SimKindPick
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Docstring

Pick-list adapter so `SimKind` can sit on the iced `pick_list`
without needing a `Display` impl on the public type.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 39–39 in `crates/oxide-app/src/library/editor/sim/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/editor/sim/mod.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/sim/mod/view.md) |
