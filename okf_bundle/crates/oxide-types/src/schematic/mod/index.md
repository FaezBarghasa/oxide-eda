# mod

## Classs

- [FillType](FillType.md) — [derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
- [Graphic](Graphic.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [HAlign](HAlign.md) — [derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
- [LabelType](LabelType.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
- [LibGraphic](LibGraphic.md) — A graphic primitive inside a library symbol, tagged with unit and body-style
- [LibPin](LibPin.md) — A pin inside a library symbol, tagged with unit and body-style.
- [LibSymbol](LibSymbol.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [Pin](Pin.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [PinDirection](PinDirection.md) — Pin electrical role.
- [PinShapeStyle](PinShapeStyle.md) — Pin graphic decoration on the symbol pin tip.
- [Point](Point.md) — [derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
- [SheetInstance](SheetInstance.md) — [derive(Debug, Clone, Default, Serialize, Deserialize)]
- [Symbol](Symbol.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [SymbolInstance](SymbolInstance.md) — [derive(Debug, Clone, Default, Serialize, Deserialize)]
- [SymbolTransform](SymbolTransform.md) — World-space placement of a parent symbol — used when folding a
- [TextProp](TextProp.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [VAlign](VAlign.md) — [derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Functions

- [apply](apply.md) — Apply transform to a library-space point.
- [apply](apply_1.md) — Apply transform to a library-space point.
- [apply_angle](apply_angle.md) — Compose a child rotation (degrees, clockwise positive in
- [apply_angle](apply_angle_1.md) — Compose a child rotation (degrees, clockwise positive in
- [apply_angle_folds_rotation_and_mirror](apply_angle_folds_rotation_and_mirror.md) — [test]
- [assert_pt](assert_pt.md)
- [circumcircle](circumcircle.md) — Circle through three non-collinear points — converts the Oxide
- [default](default.md)
- [default](default_1.md)
- [default_body_style](default_body_style.md)
- [default_true](default_true.md)
- [default_unit](default_unit.md)
- [empty](empty.md)
- [empty](empty_1.md)
- [from_symbol](from_symbol.md) — Build from a placed `Symbol`.
- [from_symbol](from_symbol_1.md) — Build from a placed `Symbol`.
- [identity_flips_library_y_up_to_schematic_y_down](identity_flips_library_y_up_to_schematic_y_down.md) — [test]
- [mirror_x_flips_the_y_output_mirror_y_flips_the_x](mirror_x_flips_the_y_output_mirror_y_flips_the_x.md) — [test]
- [new](new.md)
- [new](new_1.md)
- [origin_translates_the_result](origin_translates_the_result.md) — [test]
- [rejects_exactly_collinear_points](rejects_exactly_collinear_points.md) — [test]
- [rotation_90_turns_the_axes](rotation_90_turns_the_axes.md) — [test]
- [rotation_and_mirror_compose](rotation_and_mirror_compose.md) — [test]
- [solves_a_simple_right_triangle](solves_a_simple_right_triangle.md) — [test]
- [solves_near_collinear_triangle_in_the_drifted_epsilon_band](solves_near_collinear_triangle_in_the_drifted_epsilon_band.md) — Regression for the drifted-epsilon bug: a near-collinear triangle
- [xform](xform.md)
