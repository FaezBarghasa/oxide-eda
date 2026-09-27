# identity

## Classs

- [ComponentClass](ComponentClass.md) — Component class — picks the parameter template ("resistor", "opamp", …).
- [InternalPn](InternalPn.md) — Library-internal part number. Unique within a library; user-renameable.
- [Mpn](Mpn.md) — Manufacturer part number — moves between revisions when vendor reissues.
- [RowId](RowId.md) — Stable row identifier — UUIDv7 for time-orderability. Newtype wraps

## Functions

- [as_str](as_str.md)
- [as_str](as_str_1.md)
- [as_str](as_str_2.md)
- [as_str](as_str_3.md)
- [as_str](as_str_4.md)
- [as_str](as_str_5.md)
- [as_uuid](as_uuid.md) — Borrow the underlying UUID.
- [as_uuid](as_uuid_1.md) — Borrow the underlying UUID.
- [component_class_round_trip_and_default_is_generic](component_class_round_trip_and_default_is_generic.md) — [test]
- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [fmt](fmt_2.md)
- [fmt](fmt_3.md)
- [fmt](fmt_4.md)
- [fmt](fmt_5.md)
- [from](from.md)
- [from](from_1.md)
- [from](from_2.md)
- [from](from_3.md)
- [from_str](from_str.md)
- [from_str](from_str_1.md)
- [from_str](from_str_2.md)
- [from_str](from_str_3.md)
- [from_uuid](from_uuid.md) — Wrap an existing UUID — used when reading a row back from disk.
- [from_uuid](from_uuid_1.md) — Wrap an existing UUID — used when reading a row back from disk.
- [generic](generic.md) — Default class — applied when the user hasn't picked one yet.
- [generic](generic_1.md) — Default class — applied when the user hasn't picked one yet.
- [internal_pn_parses_via_fromstr](internal_pn_parses_via_fromstr.md) — [test]
- [internal_pn_round_trip](internal_pn_round_trip.md) — [test]
- [new](new.md) — Construct a new time-ordered RowId.
- [new](new_1.md) — Construct a new time-ordered RowId.
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [new](new_6.md)
- [new](new_7.md)
- [row_id_display_and_fromstr_match](row_id_display_and_fromstr_match.md) — [test]
- [row_id_fromstr_rejects_garbage](row_id_fromstr_rejects_garbage.md) — [test]
- [row_id_round_trip](row_id_round_trip.md) — [test]
