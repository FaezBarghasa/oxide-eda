---
okf_version: "0.2"
type: Class
title: DistributorsConfig
description: "On-disk shape. `serde(default)` lets older / partial files load"
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/DistributorsConfig
language: rust
---

# DistributorsConfig

On-disk shape. `serde(default)` lets older / partial files load

## Signature

```rust
struct DistributorsConfig
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Docstring

On-disk shape. `serde(default)` lets older / partial files load
cleanly when v0.9.1 lands new sub-tables.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `distributor_apis`

## Source
Lines 31–34 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
