# autoplace

## Classs

- [Side](Side.md) — 4. Pick the side. Score = pin_count + anchor_penalty.

## Functions

- [anchor_obstacle_count](anchor_obstacle_count.md) — Count wire endpoints + label anchors within
- [autoplace_all_marked_fields](autoplace_all_marked_fields.md)
- [autoplace_fields](autoplace_fields.md) — Pick a free side for `symbol`'s reference / value fields and write
- [graphic_extent_points](graphic_extent_points.md)
- [junction_at](junction_at.md)
- [junction_is_honoured](junction_is_honoured.md) — True when a dot at `point` would actually merge two wires — i.e. at least
- [junctions_for_wire](junctions_for_wire.md) — Every junction dot the sheet needs on account of `wire` — **both**
- [junctions_under_new_wire](junctions_under_new_wire.md) — Junction dots a newly drawn wire needs because an **existing** wire's
- [needed_junction](needed_junction.md)
- [point_on_wire_interior](point_on_wire_interior.md) — ---------------------------------------------------------------------------
- [transform_local_point](transform_local_point.md) — Apply a symbol instance's position, rotation, and mirror to a
- [wire_endpoint_count](wire_endpoint_count.md) — Number of the sheet's wires that terminate (start or end) at `point`.
- [wire_meeting_justifies_junction](wire_meeting_justifies_junction.md) — True when a genuine wire *meeting* at `point` justifies a junction dot —
