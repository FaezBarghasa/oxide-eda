# surface

## Classs

- [PdfSurface](PdfSurface.md) — `PdfSurface` emits PDF content-stream operators into a buffer.
- [RgbColor](RgbColor.md) — An RGB colour (each channel 0.0-1.0). Groups the three channels that

## Functions

- [default](default.md)
- [default](default_1.md)
- [escape_pdf_string](escape_pdf_string.md) — Escape a string for use in PDF string literals (minimal escaping).
- [escape_pdf_string_handles_special_chars](escape_pdf_string_handles_special_chars.md) — [test]
- [fill_rect](fill_rect.md)
- [fill_rect](fill_rect_1.md)
- [finish](finish.md) — Finish and return the encoded Content stream bytes.
- [finish](finish_1.md) — Finish and return the encoded Content stream bytes.
- [new](new.md)
- [new](new_1.md)
- [raw_operator](raw_operator.md) — Emit a raw operator string into the content stream.
- [raw_operator](raw_operator_1.md) — Emit a raw operator string into the content stream.
- [set_fill_color](set_fill_color.md) — Set non-stroking color (fill/text). Emits `rg` only if changed.
- [set_fill_color](set_fill_color_1.md) — Set non-stroking color (fill/text). Emits `rg` only if changed.
- [set_stroke_color](set_stroke_color.md) — Set stroke color (0.0-1.0 per channel). Emits `RG` operator only if changed.
- [set_stroke_color](set_stroke_color_1.md) — Set stroke color (0.0-1.0 per channel). Emits `RG` operator only if changed.
- [set_stroke_width](set_stroke_width.md) — Set stroke width (in points). Emits `w` operator only if changed.
- [set_stroke_width](set_stroke_width_1.md) — Set stroke width (in points). Emits `w` operator only if changed.
- [stroke_line](stroke_line.md) — Stroke a line from (x1, y1) to (x2, y2).
- [stroke_line](stroke_line_1.md) — Stroke a line from (x1, y1) to (x2, y2).
- [stroke_rect](stroke_rect.md) — Stroke a rectangle outline (top-left at (x, y), width w, height h).
- [stroke_rect](stroke_rect_1.md) — Stroke a rectangle outline (top-left at (x, y), width w, height h).
- [surface_tracks_stroke_color](surface_tracks_stroke_color.md) — [test]
- [text_at](text_at.md) — Emit text at (x, y) with given font, size (pt), and string.
- [text_at](text_at_1.md) — Emit text at (x, y) with given font, size (pt), and string.
- [text_at_rotated](text_at_rotated.md) — Emit rotated text at (x, y) with a text matrix.
- [text_at_rotated](text_at_rotated_1.md) — Emit rotated text at (x, y) with a text matrix.
- [write_operator](write_operator.md) — Write raw PDF operator bytes.
- [write_operator](write_operator_1.md) — Write raw PDF operator bytes.
