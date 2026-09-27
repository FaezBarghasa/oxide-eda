# math

## Functions

- [add](add.md) — Component-wise addition `a + b`.
- [add_diag](add_diag.md) — In-place diagonal add: `A[i][i] += λ` for all `i`. Used by LM to
- [add_diag_basic](add_diag_basic.md) — [test]
- [approx_eq](approx_eq.md)
- [axpy](axpy.md) — In-place AXPY: `y[i] += α · x[i]` for all `i`. Panics on length
- [axpy_in_place](axpy_in_place.md) — [test]
- [axpy_length_mismatch_panics](axpy_length_mismatch_panics.md) — [test]
- [cross](cross.md) — 2D scalar cross product `a × b = a.x·b.y − a.y·b.x`. Returns the
- [distance](distance.md) — Euclidean distance between two points.
- [dot](dot.md) — Dot product `a · b = a.x·b.x + a.y·b.y`.
- [dot_cross_basics](dot_cross_basics.md) — [test]
- [matmul_ata](matmul_ata.md) — `Aᵀ · A` for an `m × n` matrix `A`. Returns the `n × n` Gram
- [matmul_ata_2x2](matmul_ata_2x2.md) — [test]
- [matmul_ata_is_symmetric](matmul_ata_is_symmetric.md) — [test]
- [matvec](matvec.md) — `y = A · x` for an `m × n` matrix `A` (row-major, `Vec<Vec<f64>>`).
- [matvec_2x3](matvec_2x3.md) — [test]
- [matvec_t](matvec_t.md) — `y = Aᵀ · x` for an `m × n` matrix `A` (row-major). Returns a
- [matvec_t_3x2](matvec_t_3x2.md) — [test]
- [norm](norm.md) — Euclidean norm `|v| = sqrt(v.x² + v.y²)`. Uses [`f64::hypot`]
- [norm_distance_basics](norm_distance_basics.md) — [test]
- [norm_sq](norm_sq.md) — `|x|²` for a flat vector. The LM convergence test compares this
- [norm_sq_2](norm_sq_2.md) — Squared Euclidean norm `|v|² = v.x² + v.y²`. Avoids the `sqrt`
- [norm_sq_and_norm_vec](norm_sq_and_norm_vec.md) — [test]
- [norm_vec](norm_vec.md) — `|x|` for a flat vector.
- [scale](scale.md) — Scalar multiply `a · v`.
- [sub](sub.md) — Component-wise subtraction `a − b`.
- [sub_add_scale_compose](sub_add_scale_compose.md) — [test]
- [wrap_to_pi](wrap_to_pi.md) — Wrap an angle into the principal range `(−π, π]`.
- [wrap_to_pi_basic_cases](wrap_to_pi_basic_cases.md) — [test]
