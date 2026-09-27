---
okf_version: "0.2"
type: Function
title: is_target
description: "Power ports (`#PWR`, `#FLG`, `is_power`) aren't designators — they're net"
resource: crates/oxide-app/src/app/view/dialogs/annotate_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate_preview/is_target
language: rust
---

# is_target

Power ports (`#PWR`, `#FLG`, `is_power`) aren't designators — they're net

## Signature

```rust
fn is_target(sym: &Symbol) -> bool
```

## Docstring

Power ports (`#PWR`, `#FLG`, `is_power`) aren't designators — they're net
anchors whose "reference" is the net name.

## Source
Lines 21–23 in `crates/oxide-app/src/app/view/dialogs/annotate_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate_preview](/crates/oxide-app/src/app/view/dialogs/annotate_preview.md) |
| called_by | [preview_project_annotations](/crates/oxide-app/src/app/view/dialogs/annotate_preview/preview_project_annotations.md) |
| called_by | [seed_counters](/crates/oxide-app/src/app/view/dialogs/annotate_preview/seed_counters.md) |
