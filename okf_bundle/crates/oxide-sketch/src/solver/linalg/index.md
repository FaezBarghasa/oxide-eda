# linalg

## Classs

- [LinAlgError](LinAlgError.md) — Errors returned by the dense LU solver.
- [LuDecomposition](LuDecomposition.md) — Bundled LU factorisation: the packed `LU` matrix and the row-pivot
- [QrDecomposition](QrDecomposition.md) — Result of a Householder QR factorisation: `R` is the upper-

## Functions

- [lu_decompose](lu_decompose.md) — Compute the LU decomposition in place with partial (row) pivoting.
- [lu_solve](lu_solve.md) — Forward + back substitution given an LU-decomposed matrix and a
- [new](new.md) — Factor a square matrix into its packed LU form. The input is
- [new](new_1.md) — Factor a square matrix into its packed LU form. The input is
- [new](new_2.md) — Factor an `m × n` matrix `A` using Householder reflections.
- [new](new_3.md) — Factor an `m × n` matrix `A` using Householder reflections.
- [rank](rank.md) — Numerical rank: count of diagonal entries `|R[i][i]| > tol`.
- [rank](rank_1.md) — Numerical rank: count of diagonal entries `|R[i][i]| > tol`.
- [solve](solve.md) — Solve `A x = b` via partial-pivot LU.
- [solve](solve_1.md) — Solve `A x = b` against the cached factorisation.
- [solve](solve_2.md) — Solve `A x = b` against the cached factorisation.
