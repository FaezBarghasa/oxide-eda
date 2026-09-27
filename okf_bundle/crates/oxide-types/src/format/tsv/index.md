# tsv

## Functions

- [decode_cell](decode_cell.md) — Decode a single TSV cell. `""` and a bare `-` return empty (the
- [encode_cell](encode_cell.md) — Encode a single TSV cell. Empty strings emit `""` so column
- [escape_tsv_body_for_toml](escape_tsv_body_for_toml.md) — Escape a rendered TSV block so it can be embedded inside a TOML
- [format_f64](format_f64.md) — Format an `f64` for TSV: trailing zeros stripped to keep diffs
- [parse_f64](parse_f64.md)
- [parse_i64](parse_i64.md) — ---------------------------------------------------------------------------
- [parse_tsv_block](parse_tsv_block.md) — Parse a TSV block: validate the header against `R::columns()`,
- [parse_uuid](parse_uuid.md)
- [split_row](split_row.md) — Split a TSV row on whitespace, honouring `"` quoting so cells
- [write_tsv_block](write_tsv_block.md) — Write a TSV block: header row + one row per item, columns aligned
- [write_tsv_section](write_tsv_section.md) — ---------------------------------------------------------------------------
