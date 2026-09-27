---
okf_version: "0.2"
type: Function
title: str_to_source
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/str_to_source
language: rust
---

# str_to_source

## Signature

```rust
fn str_to_source(s: &str) -> Option<DistributorSource>
```

## Source
Lines 85–96 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| called_by | [load_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/load_preferred_order_at.md) |
