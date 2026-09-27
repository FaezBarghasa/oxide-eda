# numeric_input

## Classs

- [OptionalNumberEdit](OptionalNumberEdit.md) — What the user's text actually asked for.

## Functions

- [empty_field_clears_the_stored_value](empty_field_clears_the_stored_value.md) — [test]
- [fp_parse_optional_number](fp_parse_optional_number.md) — Read an unbounded optional-number field (`hole_rotation_deg`).
- [fp_parse_optional_number_in](fp_parse_optional_number_in.md) — Read a range-limited optional-number field (`corner_radius_pct`).
- [fp_resolve_optional_number](fp_resolve_optional_number.md) — Resolve a parsed edit into what to store, reporting anything the
- [in_range_number_is_stored_as_typed](in_range_number_is_stored_as_typed.md) — [test]
- [non_finite_numbers_are_unreadable](non_finite_numbers_are_unreadable.md) — [test]
- [out_of_range_number_clamps_instead_of_clearing](out_of_range_number_clamps_instead_of_clearing.md) — #599 — `60` used to erase the stored 25 and drop the key from the
- [resolve](resolve.md)
- [unbounded_field_keeps_negative_and_large_values](unbounded_field_keeps_negative_and_large_values.md) — The unbounded rotation field keeps every finite value, including
- [unreadable_text_refuses_the_write](unreadable_text_refuses_the_write.md) — #599 — the comma-decimal keyboard case. Refusing the write is the
