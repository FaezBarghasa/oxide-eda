# room

## Classs

- [Room](Room.md) — Definition of a 2D PCB Room grouping footprints, routing, and zones.
- [RoomCopyOptions](RoomCopyOptions.md) — Options controlling which elements are formatted/replicated when copying room formats.
- [RoomManager](RoomManager.md) — Manages PCB rooms and multi-channel format replication across repeated channels.

## Functions

- [add_room](add_room.md)
- [add_room](add_room_1.md)
- [anchor](anchor.md) — Computes the centroid/anchor of the room polygon.
- [anchor](anchor_1.md) — Computes the centroid/anchor of the room polygon.
- [auto_assign_footprints](auto_assign_footprints.md) — Auto-assigns footprints on the board to rooms if their origin falls within room boundaries.
- [auto_assign_footprints](auto_assign_footprints_1.md) — Auto-assigns footprints on the board to rooms if their origin falls within room boundaries.
- [contains_point](contains_point.md) — Checks if a 2D point lies within the room's boundary polygon (ray-casting algorithm).
- [contains_point](contains_point_1.md) — Checks if a 2D point lies within the room's boundary polygon (ray-casting algorithm).
- [copy_room_format](copy_room_format.md) — Copies the relative placement, orientation, routing traces, and vias from a source room to target rooms.
- [copy_room_format](copy_room_format_1.md) — Copies the relative placement, orientation, routing traces, and vias from a source room to target rooms.
- [default](default.md)
- [default](default_1.md)
- [find_room](find_room.md)
- [find_room](find_room_1.md)
- [find_room_mut](find_room_mut.md)
- [find_room_mut](find_room_mut_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [strip_channel_suffix](strip_channel_suffix.md) — Helper function to strip channel numbers/suffixes (e.g. "R1_CH2" -> "R1", "U1_3" -> "U1", "C1" -> "C1").
- [test_copy_room_format](test_copy_room_format.md) — [test]
- [test_room_contains_point_and_anchor](test_room_contains_point_and_anchor.md) — [test]
