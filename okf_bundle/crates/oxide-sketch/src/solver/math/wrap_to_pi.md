---
okf_version: "0.2"
type: Function
title: wrap_to_pi
description: "Wrap an angle into the principal range `(−π, π]`."
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/wrap_to_pi
language: rust
---

# wrap_to_pi

Wrap an angle into the principal range `(−π, π]`.

## Signature

```rust
pub fn wrap_to_pi(theta: f64) -> f64
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Wrap an angle into the principal range `(−π, π]`.

Computed as `θ − 2π · round(θ / 2π)` and then nudged so a value
at exactly `+π` stays `+π` (the half-open interval lives on the
negative side). This keeps the Angle constraint's residual
continuous across a sketch that crosses the ±π branch cut so the
LM driver sees a well-formed derivative instead of a 2π jump.
[inline]

## Source
Lines 98–108 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
