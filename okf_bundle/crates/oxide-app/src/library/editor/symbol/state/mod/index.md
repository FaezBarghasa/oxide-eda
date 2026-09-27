# mod

## Classs

- [BoxSelectKind](BoxSelectKind.md) — Box selection mode — determined by drag direction.
- [FieldKey](FieldKey.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
- [GraphicHandle](GraphicHandle.md) — Resize-handle identity for a placed [`SymbolGraphic`]. Each
- [GraphicRotationPivotMode](GraphicRotationPivotMode.md) — Pivot mode for Symbol graphic rotation.
- [PinKind](PinKind.md) — Coarse pin classification — kept independent of the canonical
- [SymActiveBarMenu](SymActiveBarMenu.md) — v0.13 — SchLib editor active-bar dropdown menu identifier. One
- [SymbolFilterKind](SymbolFilterKind.md) — [derive(Debug, Clone, Copy, PartialEq, Eq)]
- [SymbolSelection](SymbolSelection.md) — Selected element on the Symbol canvas — drives delete + drag and
- [SymbolSelectionFilter](SymbolSelectionFilter.md) — v0.13 — Per-kind selectable flags for the SchLib editor.

## Functions

- [add_pin](add_pin.md) — Add a pin at the given canvas coordinates and return its index in
- [common_graphic_part_number](common_graphic_part_number.md) — The single `part_number` every graphic `indices` names shares, or
- [default](default.md)
- [default](default_1.md)
- [delete_unit](delete_unit.md) — Delete sub-part `part` from the symbol: drop its pins, renumber
- [from_ai_stub](from_ai_stub.md)
- [from_ai_stub](from_ai_stub_1.md)
- [get](get.md)
- [get](get_1.md)
- [graphic_is_selected](graphic_is_selected.md) — Whether the graphic at `idx` is part of `sel` — single source of
- [graphic_on_part](graphic_on_part.md) — A graphic is visible/editable on `active_part` when it is shared
- [handle_interaction](handle_interaction.md) — Map a [`GraphicHandle`] to the mouse cursor that should be shown
- [join_source_indices](join_source_indices.md) — Graphic indices the current selection names individually. Shared
- [label](label.md)
- [label](label_1.md)
- [max_part_number](max_part_number.md) — Highest declared part number across every pin on `sym`, reconciled
- [next_pin_number](next_pin_number.md) — Pick the next integer pin number — one above the highest numeric
- [pin_on_part](pin_on_part.md) — A pin is visible/editable on `active_part` when it is shared (Part
- [polygon_centroid](polygon_centroid.md) — Centroid of a Polygon graphic — the shared anchor definition used
- [polygon_vertex_mean](polygon_vertex_mean.md) — Plain vertex mean — [`polygon_centroid`]'s degenerate-ring fallback.
- [selected_is_alignable](selected_is_alignable.md) — Whether [`align_selected_to_grid`] would actually snap anything for
- [selected_is_deletable](selected_is_deletable.md) — Whether [`delete_selected`] would actually remove anything for this
- [selection_has_enough_join_sources](selection_has_enough_join_sources.md) — Whether `indices` names enough sources to plausibly close into a
- [selection_is_join_eligible](selection_is_join_eligible.md) — Whether the current selection is eligible for "Join into Polygon":
- [selection_kinds_are_line_or_arc](selection_kinds_are_line_or_arc.md) — Whether every graphic `indices` names is a `Line` or an `Arc` — the
- [toggle](toggle.md)
- [toggle](toggle_1.md)
