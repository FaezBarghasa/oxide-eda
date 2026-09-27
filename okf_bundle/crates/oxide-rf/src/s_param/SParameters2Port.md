---
okf_version: "0.2"
type: Class
title: SParameters2Port
description: "2-Port Scattering Parameters Matrix ($S_{11}, S_{21}, S_{12}, S_{22}$)."
resource: crates/oxide-rf/src/s_param.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:31Z"
concept_id: crates/oxide-rf/src/s_param/SParameters2Port
language: rust
---

# SParameters2Port

2-Port Scattering Parameters Matrix ($S_{11}, S_{21}, S_{12}, S_{22}$).

## Signature

```rust
pub struct SParameters2Port
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

2-Port Scattering Parameters Matrix ($S_{11}, S_{21}, S_{12}, S_{22}$).
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `freq_hz`
- `s11`
- `s21`
- `s12`
- `s22`
- `z0`

## Source
Lines 83–90 in `crates/oxide-rf/src/s_param.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [s_param](/crates/oxide-rf/src/s_param.md) |
