# unit

## Classs

- [Quantity](Quantity.md) — A scalar value paired with a [`Unit`].
- [Unit](Unit.md) — A unit attached to a [`Quantity`].
- [UnitError](UnitError.md) — Errors produced by the unit parser and [`Quantity`] conversions.
- [UnitFamily](UnitFamily.md) — Group of compatible units.

## Functions

- [angle](angle.md) — Construct an angle quantity in radians.
- [angle](angle_1.md) — Construct an angle quantity in radians.
- [as_count](as_count.md) — Return the raw scalar if this quantity is dimensionless.
- [as_count](as_count_1.md) — Return the raw scalar if this quantity is dimensionless.
- [as_mm](as_mm.md) — Convert to millimetres. Errors with [`UnitError::WrongFamily`]
- [as_mm](as_mm_1.md) — Convert to millimetres. Errors with [`UnitError::WrongFamily`]
- [as_rad](as_rad.md) — Convert to radians. Errors with [`UnitError::WrongFamily`]
- [as_rad](as_rad_1.md) — Convert to radians. Errors with [`UnitError::WrongFamily`]
- [count](count.md) — Construct a dimensionless count.
- [count](count_1.md) — Construct a dimensionless count.
- [family](family.md) — Family this unit belongs to. Conversions between different
- [family](family_1.md) — Family this unit belongs to. Conversions between different
- [length](length.md) — Construct a length quantity in millimetres.
- [length](length_1.md) — Construct a length quantity in millimetres.
- [parse_quantity](parse_quantity.md) — Parse a string like `"0.5mm"`, `"100 mil"`, `"90deg"`, or `"16"`.
