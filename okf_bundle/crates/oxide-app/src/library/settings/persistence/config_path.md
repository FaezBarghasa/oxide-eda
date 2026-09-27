---
okf_version: "0.2"
type: Function
title: config_path
description: "Resolve `<config_dir>/oxide/distributors.toml`. Returns `None`"
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/config_path
language: rust
---

# config_path

Resolve `<config_dir>/oxide/distributors.toml`. Returns `None`

## Signature

```rust
pub fn config_path() -> Option<PathBuf>
```

## Visibility

- `pub`

## Docstring

Resolve `<config_dir>/oxide/distributors.toml`. Returns `None`
when the platform refuses to hand us a config dir (rare; e.g. some
sandboxed CI runners). Tests override via [`config_path_for_dir`].

## Source
Lines 101–103 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| calls | [config_root](/crates/oxide-app/src/config_root/config_root.md) |
| called_by | [load_preferred_order](/crates/oxide-app/src/library/settings/persistence/load_preferred_order.md) |
| called_by | [save_preferred_order](/crates/oxide-app/src/library/settings/persistence/save_preferred_order.md) |
