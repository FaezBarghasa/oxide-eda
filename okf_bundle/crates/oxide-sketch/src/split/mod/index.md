# mod

## Classs

- [SplitCtx](SplitCtx.md) — Bundled split parameters threaded through constraint / reference
- [SplitError](SplitError.md) — Failure modes for [`split_line`]. On every variant `sketch` is left
- [SplitResult](SplitResult.md) — Result of a successful [`split_line`] — the new mid `Point` and the
- [ValidatedSplit](ValidatedSplit.md) — Read-only results of [`validate_split`]'s checks — the retired

## Functions

- [build_split_entities](build_split_entities.md) — Mint the mid `Point` and the two replacement `Line`s, each
- [commit_split](commit_split.md) — Rewrite every other reference to `ctx.line` (constraints, arrays,
- [entity_point_xy](entity_point_xy.md) — Coordinates of a `Point` entity, or `None` if `id` doesn't resolve
- [mm_distance](mm_distance.md) — Euclidean distance between two `(x, y)` pairs in mm.
- [split_line](split_line.md) — Split the `Line` entity `line` at parameter `t` (`0.0` = start,
- [validate_split](validate_split.md) — Validate `line` / `t` against read-only lookups only — resolves the
