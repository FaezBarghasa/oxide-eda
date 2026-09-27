---
okf_version: "0.2"
type: Class
title: GlobalLibraryEntry
description: "One row in `global_libraries.toml`. Mirrors the plan §3 schema —"
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/GlobalLibraryEntry
language: rust
---

# GlobalLibraryEntry

One row in `global_libraries.toml`. Mirrors the plan §3 schema —

## Signature

```rust
pub struct GlobalLibraryEntry
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One row in `global_libraries.toml`. Mirrors the plan §3 schema —
`remote` and `auto_pull` are forward-compat hooks for the
upcoming "fetch once a day" Global library refresh; v0.9 uses
only `path`.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `path`
- `remote`
- `auto_pull`

## Source
Lines 31–37 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
