---
okf_version: "0.2"
type: Function
title: point_on_segment
description: "Strict on-wire test: the snapped click"
resource: crates/oxide-app/src/app/handlers/canvas/clicked.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/clicked/point_on_segment_1
language: rust
---

# point_on_segment

Strict on-wire test: the snapped click

## Signature

```rust
fn point_on_segment(
                        px: f64,
                        py: f64,
                        ax: f64,
                        ay: f64,
                        bx: f64,
                        by: f64,
                    ) -> bool
```

## Docstring

Strict on-wire test: the snapped click
point must lie on a wire segment (point-
to-segment distance < 0.05 mm). The
general `hit_test` uses a 1.5 mm
tolerance so parallel wires on adjacent
grid points could each register; for
net-colour painting we only want the
wire directly under the pen.

## Source
Lines 95–118 in `crates/oxide-app/src/app/handlers/canvas/clicked.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [clicked](/crates/oxide-app/src/app/handlers/canvas/clicked.md) |
