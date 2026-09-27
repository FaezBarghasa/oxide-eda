---
okf_version: "0.2"
type: Function
title: corrupt_file_returns_default_order
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
concept_id: crates/oxide-app/src/library/settings/persistence/corrupt_file_returns_default_order
language: rust
---

# corrupt_file_returns_default_order

[test]

## Signature

```rust
fn corrupt_file_returns_default_order()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 263–270 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| calls | [config_path_for_dir](/crates/oxide-app/src/library/settings/persistence/config_path_for_dir.md) |
| calls | [load_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/load_preferred_order_at.md) |
