# mod

## Classs

- [ComponentType](ComponentType.md) — Altium "Component Type" — drives BOM rules and schematic
- [PinDirection](PinDirection.md) — Electrical role of a pin — drives ERC and BOM rules.
- [PinOrientation](PinOrientation.md) — Pin orientation — which direction the pin extends from the body.
- [PinSymbolKind](PinSymbolKind.md) — Decorative IEEE-style modifier glyph attached to a pin's symbol
- [Symbol](Symbol.md) — Reusable schematic primitive. Bound by a `Component::symbol_ref`.
- [SymbolFile](SymbolFile.md) — Multi-symbol `.snxsym` container — Altium SchLib parity. One file
- [SymbolFileError](SymbolFileError.md) — Error variants for [`SymbolFile`] parsers + serialisers.
- [SymbolFileWire](SymbolFileWire.md) — On-disk wire shape. Mirrors [`SymbolFile`] but each [`Symbol`]'s
- [SymbolGraphic](SymbolGraphic.md) — One graphic on the symbol body.
- [SymbolGraphicKind](SymbolGraphicKind.md) — Drawing primitive kinds — the geometry of the symbol body.
- [SymbolPin](SymbolPin.md) — One symbol pin.
- [SymbolWire](SymbolWire.md) — [derive(Serialize, Deserialize)]

## Functions

- [default_comment](default_comment.md)
- [default_designator](default_designator.md)
- [default_format](default_format.md)
- [default_part_count](default_part_count.md)
- [default_part_number](default_part_number.md)
- [default_version](default_version.md)
- [default_visibility_true](default_visibility_true.md)
- [empty](empty.md) — Empty symbol scaffold used by New Symbol flows.
- [empty](empty_1.md) — Empty symbol scaffold used by New Symbol flows.
- [from_bytes](from_bytes.md) — Decode bytes as UTF-8 and parse via [`SymbolFile::from_toml_str`].
- [from_bytes](from_bytes_1.md) — Decode bytes as UTF-8 and parse via [`SymbolFile::from_toml_str`].
- [from_symbol](from_symbol.md) — Build a new container holding a single symbol — what the
- [from_symbol](from_symbol_1.md) — Build a new container holding a single symbol — what the
- [from_toml_str](from_toml_str.md) — Parse the TOML+TSV wire format. The format-token check accepts
- [from_toml_str](from_toml_str_1.md) — Parse the TOML+TSV wire format. The format-token check accepts
- [get_symbol](get_symbol.md) — Locate a symbol by UUID within this file.
- [get_symbol](get_symbol_1.md) — Locate a symbol by UUID within this file.
- [get_symbol_mut](get_symbol_mut.md) — Locate a symbol by UUID within this file (mutable).
- [get_symbol_mut](get_symbol_mut_1.md) — Locate a symbol by UUID within this file (mutable).
- [migrate_legacy_arc](migrate_legacy_arc.md) — Load-time self-heal for two pre-normalization `Arc`-authoring bugs.
- [new](new.md) — Convenience constructor for plumb-default tests + scaffolding.
- [new](new_1.md) — Convenience constructor for plumb-default tests + scaffolding.
- [normalize_arc_endpoints_deg](normalize_arc_endpoints_deg.md) — Normalise an `Arc`'s `start_deg`/`end_deg` into this codebase's
- [to_toml_string](to_toml_string.md) — Serialise to canonical TOML+TSV. Pin lists become
- [to_toml_string](to_toml_string_1.md) — Serialise to canonical TOML+TSV. Pin lists become
- [upsert](upsert.md) — Replace `symbol` in the container — matches by `symbol.uuid`.
- [upsert](upsert_1.md) — Replace `symbol` in the container — matches by `symbol.uuid`.
