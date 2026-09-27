---
okf_version: "0.2"
type: Function
title: default_order
description: "Default order matches `DistributorSettings::default()`."
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/default_order
language: rust
---

# default_order

Default order matches `DistributorSettings::default()`.

## Signature

```rust
fn default_order() -> Vec<DistributorSource>
```

## Docstring

Default order matches `DistributorSettings::default()`.

## Source
Lines 62–69 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| called_by | [default_order_strs](/crates/oxide-app/src/library/settings/persistence/default_order_strs.md) |
| called_by | [load_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/load_preferred_order_at.md) |
