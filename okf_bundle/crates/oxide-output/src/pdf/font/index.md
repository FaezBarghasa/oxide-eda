# font

## Classs

- [FontCatalog](FontCatalog.md) — A catalog of fonts used in the PDF, mapped to their pdf-writer Refs.
- [PdfFont](PdfFont.md) — Embedded font variants, backed by TTF bytes.

## Functions

- [alias](alias.md) — Short alias used inside content streams (`/F1 9 Tf ...`). Matches the
- [alias](alias_1.md) — Short alias used inside content streams (`/F1 9 Tf ...`). Matches the
- [aliases_and_standard_fallbacks](aliases_and_standard_fallbacks.md) — [test]
- [all_fonts_have_metrics](all_fonts_have_metrics.md) — [test]
- [base_name](base_name.md) — PostScript base name for this font (used in /BaseFont). Matches the
- [base_name](base_name_1.md) — PostScript base name for this font (used in /BaseFont). Matches the
- [base_names_correct](base_names_correct.md) — [test]
- [best_alias_for_text](best_alias_for_text.md) — Choose the most suitable alias for the given text.
- [embeds_roboto_font](embeds_roboto_font.md) — [test]
- [face](face.md) — Parse the TTF face for metadata (ascent, descent, bbox, etc.).
- [face](face_1.md) — Parse the TTF face for metadata (ascent, descent, bbox, etc.).
- [font_bytes](font_bytes.md) — Retrieve the embedded TTF bytes for this font.
- [font_bytes](font_bytes_1.md) — Retrieve the embedded TTF bytes for this font.
- [font_bytes_are_valid_ttf](font_bytes_are_valid_ttf.md) — [test]
- [font_bytes_embedded](font_bytes_embedded.md) — [test]
- [font_catalog_get_retrieves](font_catalog_get_retrieves.md) — [test]
- [font_catalog_registers_unique](font_catalog_registers_unique.md) — [test]
- [font_data](font_data.md) — Get all embedded font bytes for later embedding. Maps font to its TTF bytes.
- [font_data](font_data_1.md) — Get all embedded font bytes for later embedding. Maps font to its TTF bytes.
- [font_for_alias](font_for_alias.md) — Resolve alias (`F1`..`F4`) to PdfFont.
- [font_style_maps_to_embedded_variants](font_style_maps_to_embedded_variants.md) — [test]
- [fonts_parse_successfully](fonts_parse_successfully.md) — [test]
- [for_style](for_style.md) — Map template FontStyle to the appropriate font.
- [for_style](for_style_1.md) — Map template FontStyle to the appropriate font.
- [get](get.md) — Get the Ref for a registered font, or None.
- [get](get_1.md) — Get the Ref for a registered font, or None.
- [glyph_coverage](glyph_coverage.md)
- [iter](iter.md) — Iterate over all registered fonts.
- [iter](iter_1.md) — Iterate over all registered fonts.
- [new](new.md)
- [new](new_1.md)
- [register](register.md) — Register a font, returning its Ref. Same font registered twice returns
- [register](register_1.md) — Register a font, returning its Ref. Same font registered twice returns
- [sanitize_pdf_text](sanitize_pdf_text.md) — Convert text to a PDF-safe Latin-1-ish representation so standard-14
- [standard_ps_name](standard_ps_name.md) — PDF standard-14 Type1 font name we fall back to while full Type0
- [standard_ps_name](standard_ps_name_1.md) — PDF standard-14 Type1 font name we fall back to while full Type0
- [text_advance_pt](text_advance_pt.md) — Approximate text advance using embedded TTF metrics at the given size.
