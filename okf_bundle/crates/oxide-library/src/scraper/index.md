# scraper

## Classs

- [ComponentScraper](ComponentScraper.md) — Component Web Scraper and Library Ingestor with Anti-Bot / AI Detection Bypass.
- [PackageType](PackageType.md) — Package geometry style used for footprint synthesis.
- [ScrapedComponent](ScrapedComponent.md) — Scraped component summary from internet search providers.

## Functions

- [default](default.md)
- [default](default_1.md)
- [download_datasheet](download_datasheet.md) — Downloads a datasheet PDF over HTTP with stealth headers and calculates its SHA-256 hash.
- [download_datasheet](download_datasheet_1.md) — Downloads a datasheet PDF over HTTP with stealth headers and calculates its SHA-256 hash.
- [extract_trailing_digits](extract_trailing_digits.md)
- [from_name](from_name.md) — Detects package style from package name string (e.g. "0805", "SOIC-8", "SOT-23-3").
- [from_name](from_name_1.md) — Detects package style from package name string (e.g. "0805", "SOIC-8", "SOT-23-3").
- [import_to_library](import_to_library.md) — Imports a scraped component, its synthesized symbol, footprint, and downloaded datasheet
- [import_to_library](import_to_library_1.md) — Imports a scraped component, its synthesized symbol, footprint, and downloaded datasheet
- [new](new.md) — Creates a stealth scraper client configured with TLS fingerprinting hygiene,
- [new](new_1.md) — Creates a stealth scraper client configured with TLS fingerprinting hygiene,
- [persist_datasheet](persist_datasheet.md) — Saves the downloaded datasheet bytes into the library's `datasheets/` directory.
- [persist_datasheet](persist_datasheet_1.md) — Saves the downloaded datasheet bytes into the library's `datasheets/` directory.
- [search](search.md) — Searches online component distributors for the given query/MPN with stealth headers and anti-detection.
- [search](search_1.md) — Searches online component distributors for the given query/MPN with stealth headers and anti-detection.
- [synthesize_footprint](synthesize_footprint.md) — Synthesizes an IPC-compliant Footprint based on package geometry.
- [synthesize_symbol](synthesize_symbol.md) — Synthesizes a schematic Symbol with bounding box graphics and pins.
- [test_package_detection](test_package_detection.md) — [test]
- [test_scraper_search_fallback](test_scraper_search_fallback.md) — [test]
- [test_synthesize_footprint_chip](test_synthesize_footprint_chip.md) — [test]
- [test_synthesize_footprint_soic](test_synthesize_footprint_soic.md) — [test]
- [test_synthesize_symbol](test_synthesize_symbol.md) — [test]
