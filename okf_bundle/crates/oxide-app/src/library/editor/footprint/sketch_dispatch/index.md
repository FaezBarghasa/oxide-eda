# sketch_dispatch

## Functions

- [apply_edit_inner](apply_edit_inner.md) — Mutates `footprint.sketch` per the edit. Idempotent on its inputs
- [apply_sketch_edit](apply_sketch_edit.md) — Apply a single [`SketchEdit`] and (if the sketch is non-trivial
- [apply_sketch_edit_with_warnings](apply_sketch_edit_with_warnings.md) — Same as [`apply_sketch_edit`] but captures any returned
- [apply_sketch_role](apply_sketch_role.md) — v0.16.2 — apply a [`RoleTag`] change to the entity at `id`.
- [apply_sketch_role_with_warnings](apply_sketch_role_with_warnings.md) — `_with_warnings` companion to [`apply_sketch_role`] — captures the
- [current_role_of](current_role_of.md) — Read the current [`RoleTag`] of an entity by inspecting which
- [set_entity_role](set_entity_role.md) — Mutates the entity's role attrs in place. Pure — no solver work.
- [solve_and_bake](solve_and_bake.md) — Resolve parameters, run LM, capture DOF, bake pads.
