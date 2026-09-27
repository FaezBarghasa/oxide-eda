---
okf_version: "0.2"
type: Function
title: arc_degree_edits_stay_in_range_and_preserve_sweep
description: "Regression: a Properties-panel arc-degree edit must persist endpoints"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/mod/arc_degree_edits_stay_in_range_and_preserve_sweep
language: rust
---

# arc_degree_edits_stay_in_range_and_preserve_sweep

Regression: a Properties-panel arc-degree edit must persist endpoints

## Signature

```rust
fn arc_degree_edits_stay_in_range_and_preserve_sweep()
```

## Decorators

- `test`

## Docstring

Regression: a Properties-panel arc-degree edit must persist endpoints
already reduced into [0, 360). A raw negative endpoint reaching disk
trips `migrate_legacy_arc` on reload (it fires on `end_deg < 0.0`) into
swapping the pair to its complement — a 270° arc silently reloads as
90°, and the migration is idempotent so the loss is unrecoverable.
[test]

## Source
Lines 861–888 in `crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_library](/crates/oxide-app/src/app/handlers/dock/sch_library/mod.md) |
| calls | [apply_graphic_field](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/apply_graphic_field.md) |
