---
okf_version: "0.2"
type: Function
title: to_matrix
description: 3×3 row-major homogeneous transform matrix.
resource: crates/oxide-types/src/anchor2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/anchor2d/to_matrix_1
language: rust
---

# to_matrix

3×3 row-major homogeneous transform matrix.

## Signature

```rust
pub fn to_matrix(&self) -> [[f64; 3]; 3]
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

3×3 row-major homogeneous transform matrix.

Maps a local-space point `p` (relative to the object origin) to world space:

```text
[wx, wy, 1] = [lx, ly, 1] * M^T
(or equivalently: world = M * [lx, ly, 1]^T)
```

Matrix: `T(pivot_world) · R(rotation_rad) · T(local_offset)`

```text
[ c  -s  (c·lo_x − s·lo_y + pw_x) ]
[ s   c  (s·lo_x + c·lo_y + pw_y) ]
[ 0   0   1                        ]
```
[must_use]

## Source
Lines 138–150 in `crates/oxide-types/src/anchor2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [anchor2d](/crates/oxide-types/src/anchor2d.md) |
