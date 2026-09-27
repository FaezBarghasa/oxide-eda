# mod

## Classs

- [ClassEntry](ClassEntry.md) — One row of the per-library class registry. `key` is the canonical
- [ColumnType](ColumnType.md) — Per-column type declared in `[tables.<name>.column_types]`.
- [ColumnTypeParseError](ColumnTypeParseError.md) — Errors from [`ColumnType::parse_token`].
- [LibraryFile](LibraryFile.md) — Top-level on-disk shape of a `.snxlib` file.
- [LibraryFileError](LibraryFileError.md) — Errors from [`LibraryFile::parse`] / [`LibraryFile::write`].
- [LibraryRow](LibraryRow.md) — One row inside a [`LibraryTable`]. Cell lookup is by column name —
- [LibrarySection](LibrarySection.md) — `[library]` block — human-readable name + description. The `library_id`
- [LibraryTable](LibraryTable.md) — Parsed in-memory view of one `[tables.<name>]` block.
- [SnxlibManifest](SnxlibManifest.md) — Manifest header — everything in a `.snxlib` *except* the

## Functions

- [cell](cell.md) — Look up `column` in `row.cells`, scoped to this table's schema.
- [cell](cell_1.md) — Look up `column` in `row.cells`, scoped to this table's schema.
- [deserialize](deserialize.md)
- [deserialize](deserialize_1.md)
- [from](from.md) — Funnel `.snxlib` parse / write failures into the adapter's error
- [from](from_1.md) — Funnel `.snxlib` parse / write failures into the adapter's error
- [parse_token](parse_token.md) — Parse a TOML wire token back to the typed enum. Errors on
- [parse_token](parse_token_1.md) — Parse a TOML wire token back to the typed enum. Errors on
- [serialize](serialize.md)
- [serialize](serialize_1.md)
- [to_token](to_token.md) — Encode the type as the string token written in TOML. Unit
- [to_token](to_token_1.md) — Encode the type as the string token written in TOML. Unit
