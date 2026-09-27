---
okf_version: "0.2"
type: Class
title: GrowthParams
description: "Grow `buffer` (and its element `capacity`) so it can hold `required`"
resource: crates/oxide-gfx/src/pipeline/growth.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/pipeline/growth/GrowthParams
language: rust
---

# GrowthParams

Grow `buffer` (and its element `capacity`) so it can hold `required`

## Signature

```rust
pub(crate) struct GrowthParams
```

## Visibility

- `pub(crate)`

## Docstring

Grow `buffer` (and its element `capacity`) so it can hold `required`
elements of `elem_size` bytes, clamped so the allocation never exceeds the
device's `max_buffer_size`.

Capacity only ever grows and is rounded up to a power of two to amortise
reallocation. When the requested element count would push the buffer past
`max_buffer_size` — a pathological board, e.g. a fully poured plane fanned
into millions of triangles — the capacity is clamped to the largest count
that fits and the caller must upload only that many elements. A clamped,
partially drawn scene is a graceful degrade; letting `create_buffer` run
with an oversize descriptor trips wgpu validation and panics the render
thread.

Returns the number of elements the caller may safely write (`== required`
unless clamped), so the caller can truncate its upload slice and set its
draw count to the returned value. Logs once per process the first time a
clamp happens.
Static growth configuration for a single instance/vertex buffer: *how*
to grow it, as opposed to `buffer`/`capacity` (its current state) or
`required` (how many elements this particular call needs). Built once
per call site in `circle.rs`/`line.rs`/`polygon.rs`/`arc.rs` and passed
by reference since `ensure_capacity` only reads it.

## Methods

- `elem_size`
- `label`
- `usage`
- `max_buffer_size`

## Source
Lines 29–34 in `crates/oxide-gfx/src/pipeline/growth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [growth](/crates/oxide-gfx/src/pipeline/growth.md) |
