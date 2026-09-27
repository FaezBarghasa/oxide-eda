---
okf_version: "0.2"
type: Function
title: load_preferred_order
description: Load the persisted preferred-order list. Returns the v0.9-library-plan.md
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/load_preferred_order
language: rust
---

# load_preferred_order

Load the persisted preferred-order list. Returns the v0.9-library-plan.md

## Signature

```rust
pub fn load_preferred_order() -> Vec<DistributorSource>
```

## Visibility

- `pub`

## Docstring

Load the persisted preferred-order list. Returns the v0.9-library-plan.md
default when the file is missing/empty/corrupt — startup is
best-effort.

## Source
Lines 115–119 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| calls | [config_path](/crates/oxide-app/src/library/settings/persistence/config_path.md) |
| calls | [load_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/load_preferred_order_at.md) |
| called_by | [default](/crates/oxide-app/src/library/state/methods/default.md) |
