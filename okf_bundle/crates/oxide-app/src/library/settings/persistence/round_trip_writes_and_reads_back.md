---
okf_version: "0.2"
type: Function
title: round_trip_writes_and_reads_back
description: "[test]"
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/round_trip_writes_and_reads_back
language: rust
---

# round_trip_writes_and_reads_back

[test]

## Signature

```rust
fn round_trip_writes_and_reads_back()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 213–226 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| calls | [config_path_for_dir](/crates/oxide-app/src/library/settings/persistence/config_path_for_dir.md) |
| calls | [save_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/save_preferred_order_at.md) |
| calls | [load_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/load_preferred_order_at.md) |
