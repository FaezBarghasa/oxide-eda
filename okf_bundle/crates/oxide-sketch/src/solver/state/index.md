# state

## Classs

- [EntityIndex](EntityIndex.md) — Maps a Point's `SketchEntityId` to its `(x, y)` offset in the
- [PackedState](PackedState.md) — [derive(Debug, Clone)]

## Functions

- [arc_refs](arc_refs.md) — Resolve the centre/start/end Point IDs and CCW flag for an
- [center_of](center_of.md) — Resolve the centre Point ID for either an [`EntityKind::Arc`] or
- [circle_radius](circle_radius.md) — Look up the radius of a [`EntityKind::Circle`] from the state
- [find_entity](find_entity.md) — Lookup helper used by tests that need an [`Entity`] by ID.
- [line_endpoints](line_endpoints.md) — Resolve the start/end Point IDs for a [`EntityKind::Line`].
- [pack](pack.md) — Build a state vector from a sketch. Fixed-constrained Points are
- [point_xy](point_xy.md) — Look up a Point's current `(x, y)` — either from the state vector
