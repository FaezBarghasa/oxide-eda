---
okf_version: "0.2"
type: Module
title: math
description: Self-contained 2D vector + dense linear-algebra primitives used
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math
language: rust
---

# math

Self-contained 2D vector + dense linear-algebra primitives used

## Docstring

Self-contained 2D vector + dense linear-algebra primitives used
across the solver. Everything is stdlib-only `f64`; no external
numerics crate is taken on.

What lives here
---------------
- **2D vector primitives** (`Vec2`, `dot`, `cross`, `norm`,
`distance`, `wrap_to_pi`) — compose the Phase 2 residual
functions in `solver/residuals/*.rs` from a common vocabulary so
any algebraic update is local to one file.
- **Dense vector / matrix primitives** (`norm_sq`, `axpy`,
`matvec`, `matmul_ata`, `add_diag`) — the building blocks of
the LM normal-equation update `(JᵀJ + λI) Δx = −Jᵀr`.

The LU factorisation lives in [`crate::solver::linalg`]; this
module deliberately only carries the *primitives* so each piece
is independently testable + benchmarkable.

References:
- Hearn & Baker, *Computer Graphics with OpenGL*, ch. 5 (2D
vector geometry).
- Press et al., *Numerical Recipes* (3rd ed.), §2.1 (vector and
matrix conventions), §2.3 (Gaussian elimination — see
`crate::solver::linalg`), §15.5 (Levenberg–Marquardt — see
`crate::solver::lm`).

No third-party numerical library source has been consulted; every
formula is derived from first-principles linear algebra.

## Relationships

| Type | Target |
|------|--------|
| related | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
| related | [add](/crates/oxide-sketch/src/solver/math/add.md) |
| related | [scale](/crates/oxide-sketch/src/solver/math/scale.md) |
| related | [dot](/crates/oxide-sketch/src/solver/math/dot.md) |
| related | [cross](/crates/oxide-sketch/src/solver/math/cross.md) |
| related | [norm](/crates/oxide-sketch/src/solver/math/norm.md) |
| related | [norm_sq_2](/crates/oxide-sketch/src/solver/math/norm_sq_2.md) |
| related | [distance](/crates/oxide-sketch/src/solver/math/distance.md) |
| related | [wrap_to_pi](/crates/oxide-sketch/src/solver/math/wrap_to_pi.md) |
| related | [norm_sq](/crates/oxide-sketch/src/solver/math/norm_sq.md) |
| related | [norm_vec](/crates/oxide-sketch/src/solver/math/norm_vec.md) |
| related | [axpy](/crates/oxide-sketch/src/solver/math/axpy.md) |
| related | [matvec](/crates/oxide-sketch/src/solver/math/matvec.md) |
| related | [matvec_t](/crates/oxide-sketch/src/solver/math/matvec_t.md) |
| related | [matmul_ata](/crates/oxide-sketch/src/solver/math/matmul_ata.md) |
| related | [add_diag](/crates/oxide-sketch/src/solver/math/add_diag.md) |
| related | [approx_eq](/crates/oxide-sketch/src/solver/math/approx_eq.md) |
| related | [sub_add_scale_compose](/crates/oxide-sketch/src/solver/math/sub_add_scale_compose.md) |
| related | [dot_cross_basics](/crates/oxide-sketch/src/solver/math/dot_cross_basics.md) |
| related | [norm_distance_basics](/crates/oxide-sketch/src/solver/math/norm_distance_basics.md) |
| related | [wrap_to_pi_basic_cases](/crates/oxide-sketch/src/solver/math/wrap_to_pi_basic_cases.md) |
| related | [norm_sq_and_norm_vec](/crates/oxide-sketch/src/solver/math/norm_sq_and_norm_vec.md) |
| related | [axpy_in_place](/crates/oxide-sketch/src/solver/math/axpy_in_place.md) |
| related | [matvec_2x3](/crates/oxide-sketch/src/solver/math/matvec_2x3.md) |
| related | [matvec_t_3x2](/crates/oxide-sketch/src/solver/math/matvec_t_3x2.md) |
| related | [matmul_ata_2x2](/crates/oxide-sketch/src/solver/math/matmul_ata_2x2.md) |
| related | [matmul_ata_is_symmetric](/crates/oxide-sketch/src/solver/math/matmul_ata_is_symmetric.md) |
| related | [add_diag_basic](/crates/oxide-sketch/src/solver/math/add_diag_basic.md) |
| related | [axpy_length_mismatch_panics](/crates/oxide-sketch/src/solver/math/axpy_length_mismatch_panics.md) |
