---
okf_version: "0.2"
type: Function
title: smith_chart_point
description: Projected coordinate on a normalized Smith Chart (reflection coefficient $\Gamma = u + jv$).
resource: crates/oxide-rf/src/s_param.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:31Z"
concept_id: crates/oxide-rf/src/s_param/smith_chart_point
language: rust
---

# smith_chart_point

Projected coordinate on a normalized Smith Chart (reflection coefficient $\Gamma = u + jv$).

## Signature

```rust
impl SParameters2Port { pub fn smith_chart_point(&self) -> (f64, f64) }
```

## Visibility

- `pub`

## Docstring

Projected coordinate on a normalized Smith Chart (reflection coefficient $\Gamma = u + jv$).

## Source
Lines 122–124 in `crates/oxide-rf/src/s_param.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [s_param](/crates/oxide-rf/src/s_param.md) |
