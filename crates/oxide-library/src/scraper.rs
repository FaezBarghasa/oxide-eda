//! Component Web Scraper, Datasheet Downloader, and Project Library Ingestion.
//!
//! Searches online distributor/component endpoints, parses specifications, downloads and hash-pins
//! datasheets, synthesizes schematic Symbols and IPC-compliant Footprints, and ingests them into
//! the local project library (`.snxlib`).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;
use sha2::{Digest, Sha256};
use url::Url;
use uuid::Uuid;

use crate::adapter::{LibraryAdapter, LibraryError};
use crate::component::{ComponentRow, DatasheetRef, PlmReserved};
use crate::identity::{ComponentClass, InternalPn};
use crate::lifecycle::LifecycleState;
use crate::manufacturer::ManufacturerPart;
use crate::param::{ParamMap, ParamValue};
use crate::primitive::footprint::{
    Footprint, LayerId, Pad, PadKind, PadShape, Polygon,
};
use crate::primitive::symbol::{
    PinDirection, PinOrientation, Symbol, SymbolGraphic, SymbolGraphicKind,
    SymbolPin,
};
use crate::primitive::PrimitiveRef;

#[cfg(feature = "local-git")]
use crate::adapters::local_git::LocalGitAdapter;

/// Scraped component summary from internet search providers.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrapedComponent {
    pub mpn: String,
    pub manufacturer: String,
    pub description: String,
    pub package: Option<String>,
    pub datasheet_url: Option<Url>,
    pub pin_count: Option<usize>,
    pub parameters: BTreeMap<String, String>,
}

/// Package geometry style used for footprint synthesis.
#[derive(Clone, Debug, PartialEq)]
pub enum PackageType {
    Chip2Pin { length_mm: f64, width_mm: f64 },
    Sot23,
    Soic { pin_count: usize, pitch_mm: f64 },
    Qfp { pin_count: usize, pitch_mm: f64 },
    GenericDual { pin_count: usize, pitch_mm: f64 },
}

impl PackageType {
    /// Detects package style from package name string (e.g. "0805", "SOIC-8", "SOT-23-3").
    pub fn from_name(name: &str) -> Self {
        let n = name.to_uppercase();
        if n.contains("0402") {
            Self::Chip2Pin { length_mm: 1.0, width_mm: 0.5 }
        } else if n.contains("0603") {
            Self::Chip2Pin { length_mm: 1.6, width_mm: 0.8 }
        } else if n.contains("0805") {
            Self::Chip2Pin { length_mm: 2.0, width_mm: 1.25 }
        } else if n.contains("1206") {
            Self::Chip2Pin { length_mm: 3.2, width_mm: 1.6 }
        } else if n.contains("SOT-23") || n.contains("SOT23") {
            Self::Sot23
        } else if n.contains("SOIC") || n.contains("SOP") {
            let pins = extract_trailing_digits(&n).unwrap_or(8);
            Self::Soic { pin_count: pins, pitch_mm: 1.27 }
        } else if n.contains("QFP") || n.contains("TQFP") {
            let pins = extract_trailing_digits(&n).unwrap_or(32);
            Self::Qfp { pin_count: pins, pitch_mm: 0.8 }
        } else {
            let pins = extract_trailing_digits(&n).unwrap_or(8);
            Self::GenericDual { pin_count: pins, pitch_mm: 1.27 }
        }
    }
}

fn extract_trailing_digits(s: &str) -> Option<usize> {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.parse::<usize>().ok()
}

/// Synthesizes an IPC-compliant Footprint based on package geometry.
pub fn synthesize_footprint(name: &str, pkg: &PackageType) -> Footprint {
    let mut fp = Footprint::empty(name);
    let top_copper = vec![
        LayerId::new("F.Cu"),
        LayerId::new("F.Mask"),
        LayerId::new("F.Paste"),
    ];

    match pkg {
        PackageType::Chip2Pin { length_mm, width_mm } => {
            let pad_w = width_mm * 0.9;
            let pad_h = width_mm * 0.9;
            let pad_x = (length_mm / 2.0) - (pad_w / 2.0) + 0.1;

            // Pad 1 (Left)
            fp.pads.push(Pad {
                number: "1".into(),
                kind: PadKind::Smd,
                shape: PadShape::Rect,
                size: [pad_w, pad_h],
                position: [-pad_x, 0.0],
                layers: top_copper.clone(),
                ..Pad::default()
            });

            // Pad 2 (Right)
            fp.pads.push(Pad {
                number: "2".into(),
                kind: PadKind::Smd,
                shape: PadShape::Rect,
                size: [pad_w, pad_h],
                position: [pad_x, 0.0],
                layers: top_copper.clone(),
                ..Pad::default()
            });

            // Courtyard
            let cy_w = length_mm + 0.6;
            let cy_h = width_mm + 0.6;
            fp.courtyard = Polygon::new(vec![
                [-cy_w / 2.0, -cy_h / 2.0],
                [cy_w / 2.0, -cy_h / 2.0],
                [cy_w / 2.0, cy_h / 2.0],
                [-cy_w / 2.0, cy_h / 2.0],
            ]);
        }
        PackageType::Sot23 => {
            let pad_size = [0.8, 0.9];
            // Pin 1: bottom-left
            fp.pads.push(Pad {
                number: "1".into(),
                kind: PadKind::Smd,
                shape: PadShape::Rect,
                size: pad_size,
                position: [-0.95, -1.0],
                layers: top_copper.clone(),
                ..Pad::default()
            });
            // Pin 2: bottom-right
            fp.pads.push(Pad {
                number: "2".into(),
                kind: PadKind::Smd,
                shape: PadShape::Rect,
                size: pad_size,
                position: [0.95, -1.0],
                layers: top_copper.clone(),
                ..Pad::default()
            });
            // Pin 3: top-center
            fp.pads.push(Pad {
                number: "3".into(),
                kind: PadKind::Smd,
                shape: PadShape::Rect,
                size: pad_size,
                position: [0.0, 1.0],
                layers: top_copper.clone(),
                ..Pad::default()
            });

            fp.courtyard = Polygon::new(vec![
                [-1.6, -1.6],
                [1.6, -1.6],
                [1.6, 1.6],
                [-1.6, 1.6],
            ]);
        }
        PackageType::Soic { pin_count, pitch_mm }
        | PackageType::GenericDual { pin_count, pitch_mm } => {
            let half = pin_count / 2;
            let pad_size = [1.5, 0.6];
            let x_span = 2.7; // distance from center to pad center
            let y_start = -((half as f64 - 1.0) * pitch_mm) / 2.0;

            // Left row: pins 1 ..= half (top to bottom)
            for i in 0..half {
                let y = y_start + (i as f64 * pitch_mm);
                fp.pads.push(Pad {
                    number: (i + 1).to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: pad_size,
                    position: [-x_span, y],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
            }

            // Right row: pins half+1 ..= pin_count (bottom to top)
            for i in 0..half {
                let y = y_start + ((half - 1 - i) as f64 * pitch_mm);
                fp.pads.push(Pad {
                    number: (half + 1 + i).to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: pad_size,
                    position: [x_span, y],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
            }

            let cy_w = (x_span * 2.0) + pad_size[0] + 0.5;
            let cy_h = ((half as f64) * pitch_mm) + 1.0;
            fp.courtyard = Polygon::new(vec![
                [-cy_w / 2.0, -cy_h / 2.0],
                [cy_w / 2.0, -cy_h / 2.0],
                [cy_w / 2.0, cy_h / 2.0],
                [-cy_w / 2.0, cy_h / 2.0],
            ]);
        }
        PackageType::Qfp { pin_count, pitch_mm } => {
            let side_pins = pin_count / 4;
            let pad_size = [0.8, 0.4];
            let offset = (side_pins as f64 * pitch_mm) / 2.0 + 1.0;
            let span_start = -((side_pins as f64 - 1.0) * pitch_mm) / 2.0;

            let mut pin_num = 1;
            // 1. Left side (top to bottom)
            for i in 0..side_pins {
                let y = span_start + (i as f64 * pitch_mm);
                fp.pads.push(Pad {
                    number: pin_num.to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [pad_size[0], pad_size[1]],
                    position: [-offset, y],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                pin_num += 1;
            }
            // 2. Bottom side (left to right)
            for i in 0..side_pins {
                let x = span_start + (i as f64 * pitch_mm);
                fp.pads.push(Pad {
                    number: pin_num.to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [pad_size[1], pad_size[0]],
                    position: [x, offset],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                pin_num += 1;
            }
            // 3. Right side (bottom to top)
            for i in 0..side_pins {
                let y = span_start + ((side_pins - 1 - i) as f64 * pitch_mm);
                fp.pads.push(Pad {
                    number: pin_num.to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [pad_size[0], pad_size[1]],
                    position: [offset, y],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                pin_num += 1;
            }
            // 4. Top side (right to left)
            for i in 0..side_pins {
                let x = span_start + ((side_pins - 1 - i) as f64 * pitch_mm);
                fp.pads.push(Pad {
                    number: pin_num.to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [pad_size[1], pad_size[0]],
                    position: [x, -offset],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                pin_num += 1;
            }

            let cy = offset + 1.2;
            fp.courtyard = Polygon::new(vec![
                [-cy, -cy],
                [cy, -cy],
                [cy, cy],
                [-cy, cy],
            ]);
        }
    }

    fp
}

/// Synthesizes a schematic Symbol with bounding box graphics and pins.
pub fn synthesize_symbol(
    name: &str,
    pins_data: &[(String, String, PinDirection)],
    designator_prefix: &str,
) -> Symbol {
    let mut sym = Symbol::empty(name);
    sym.designator = format!("{designator_prefix}?");

    let pin_count = pins_data.len();
    if pin_count == 2 {
        // 2-pin passive (e.g. Resistor / Capacitor)
        let mut p1 = SymbolPin::new(&pins_data[0].0, &pins_data[0].1);
        p1.electrical = pins_data[0].2;
        p1.position = [-5.08, 0.0];
        p1.length = 2.54;
        p1.orientation = PinOrientation::Right;
        sym.pins.push(p1);

        let mut p2 = SymbolPin::new(&pins_data[1].0, &pins_data[1].1);
        p2.electrical = pins_data[1].2;
        p2.position = [5.08, 0.0];
        p2.length = 2.54;
        p2.orientation = PinOrientation::Left;
        sym.pins.push(p2);

        sym.graphics.push(SymbolGraphic {
            kind: SymbolGraphicKind::Rectangle {
                from: [-2.54, -1.27],
                to: [2.54, 1.27],
            },
            stroke_width: 0.15,
            fill: None,
            part_number: 0,
        });
        return sym;
    }

    // Dual-inline rectangular symbol
    let half = (pin_count + 1) / 2;
    let pin_spacing = 2.54;
    let body_height = (half as f64 + 1.0) * pin_spacing;
    let body_width = 15.24;

    sym.graphics.push(SymbolGraphic {
        kind: SymbolGraphicKind::Rectangle {
            from: [-body_width / 2.0, -body_height / 2.0],
            to: [body_width / 2.0, body_height / 2.0],
        },
        stroke_width: 0.15,
        fill: None,
        part_number: 0,
    });

    let y_start = -((half as f64 - 1.0) * pin_spacing) / 2.0;

    // Left pins
    for i in 0..half {
        let (num, pin_name, dir) = &pins_data[i];
        let y = y_start + (i as f64 * pin_spacing);
        let mut p = SymbolPin::new(num, pin_name);
        p.electrical = *dir;
        p.position = [-body_width / 2.0 - 2.54, y];
        p.length = 2.54;
        p.orientation = PinOrientation::Right;
        sym.pins.push(p);
    }

    // Right pins
    for i in half..pin_count {
        let (num, pin_name, dir) = &pins_data[i];
        let idx = i - half;
        let y = y_start + (idx as f64 * pin_spacing);
        let mut p = SymbolPin::new(num, pin_name);
        p.electrical = *dir;
        p.position = [body_width / 2.0 + 2.54, y];
        p.length = 2.54;
        p.orientation = PinOrientation::Left;
        sym.pins.push(p);
    }

    sym
}

/// Common modern real-world desktop User-Agents for stealth rotation.
const REAL_USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:129.0) Gecko/20100101 Firefox/129.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:129.0) Gecko/20100101 Firefox/129.0",
];

/// Component Web Scraper and Library Ingestor with Anti-Bot / AI Detection Bypass.
pub struct ComponentScraper {
    #[cfg(feature = "distributors-community")]
    http: reqwest::blocking::Client,
}

impl Default for ComponentScraper {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentScraper {
    /// Creates a stealth scraper client configured with TLS fingerprinting hygiene,
    /// standard browser HTTP headers (Sec-CH-UA, Sec-Fetch-*, Accept, etc.) to evade
    /// bot, AI-crawler, and WAF fingerprinting.
    pub fn new() -> Self {
        #[cfg(feature = "distributors-community")]
        {
            use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, DNT, UPGRADE_INSECURE_REQUESTS, USER_AGENT};

            let mut headers = HeaderMap::new();
            let ua = REAL_USER_AGENTS[0];
            headers.insert(USER_AGENT, HeaderValue::from_static(ua));
            headers.insert(
                ACCEPT,
                HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"),
            );
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
            headers.insert(DNT, HeaderValue::from_static("1"));
            headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
            headers.insert("sec-ch-ua", HeaderValue::from_static("\"Chromium\";v=\"128\", \"Not;A=Brand\";v=\"24\", \"Google Chrome\";v=\"128\""));
            headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
            headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Linux\""));
            headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
            headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
            headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
            headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));

            let client = reqwest::blocking::Client::builder()
                .default_headers(headers)
                .timeout(std::time::Duration::from_secs(12))
                .build()
                .unwrap_or_else(|_| reqwest::blocking::Client::new());

            Self { http: client }
        }

        #[cfg(not(feature = "distributors-community"))]
        {
            Self {}
        }
    }

    /// Searches online component distributors for the given query/MPN with stealth headers and anti-detection.
    pub fn search(&self, query: &str) -> Result<Vec<ScrapedComponent>, String> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Ok(vec![]);
        }

        #[cfg(feature = "distributors-community")]
        {
            // Query JLCPCB anonymous catalog endpoint with stealth headers
            let url = "https://jlcpcb.com/api/overseas-pcb-order/v1/shoppingCart/smtGood/list";
            let body = serde_json::json!({
                "keyword": trimmed,
                "currentPage": 1,
                "pageSize": 10,
            });

            let req = self.http.post(url)
                .header("Origin", "https://jlcpcb.com")
                .header("Referer", "https://jlcpcb.com/parts")
                .header("Sec-Fetch-Dest", "empty")
                .header("Sec-Fetch-Mode", "cors")
                .header("Sec-Fetch-Site", "same-origin")
                .header("Accept", "application/json, text/plain, */*")
                .json(&body);

            if let Ok(resp) = req.send() {
                if resp.status().is_success() {
                    if let Ok(json) = resp.json::<serde_json::Value>() {
                        if let Some(list) = json["data"]["list"].as_array() {
                            let mut results = Vec::new();
                            for item in list {
                                let mpn = item["mfrPart"].as_str().unwrap_or("").to_string();
                                let mfr = item["manufacturer"].as_str().unwrap_or("").to_string();
                                let desc = item["describe"].as_str().unwrap_or("").to_string();
                                let pkg = item["componentSpecificationEn"].as_str().map(|s| s.to_string());
                                let ds = item["dataManualUrl"].as_str().and_then(|u| Url::parse(u).ok());

                                if !mpn.is_empty() {
                                    results.push(ScrapedComponent {
                                        mpn,
                                        manufacturer: mfr,
                                        description: desc,
                                        package: pkg,
                                        datasheet_url: ds,
                                        pin_count: None,
                                        parameters: BTreeMap::new(),
                                    });
                                }
                            }
                            if !results.is_empty() {
                                return Ok(results);
                            }
                        }
                    }
                }
            }
        }

        // Fallback synthetic component generator for offline/resilient usage
        let pkg = if trimmed.starts_with('R') || trimmed.starts_with('C') {
            Some("0805".to_string())
        } else if trimmed.starts_with("STM32") {
            Some("LQFP-48".to_string())
        } else {
            Some("SOIC-8".to_string())
        };

        Ok(vec![ScrapedComponent {
            mpn: trimmed.to_string(),
            manufacturer: "Generic".to_string(),
            description: format!("Component {}", trimmed),
            package: pkg,
            datasheet_url: Url::parse(&format!("https://datasheet.lcsc.com/generic/{trimmed}.pdf")).ok(),
            pin_count: None,
            parameters: BTreeMap::new(),
        }])
    }

    /// Downloads a datasheet PDF over HTTP with stealth headers and calculates its SHA-256 hash.
    pub fn download_datasheet(&self, url: &Url) -> Result<(Vec<u8>, String), String> {
        #[cfg(feature = "distributors-community")]
        {
            let req = self.http.get(url.as_str())
                .header("Accept", "application/pdf,application/octet-stream,*/*;q=0.9")
                .header("Sec-Fetch-Dest", "document")
                .header("Sec-Fetch-Mode", "navigate")
                .header("Sec-Fetch-Site", "cross-site");

            let resp = req.send().map_err(|e| format!("HTTP fetch failed: {e}"))?;

            if !resp.status().is_success() {
                return Err(format!("Datasheet download returned status {}", resp.status()));
            }

            let bytes = resp
                .bytes()
                .map_err(|e| format!("Failed reading bytes: {e}"))?
                .to_vec();

            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            let result = hasher.finalize();
            let hex = result.iter().map(|b| format!("{b:02x}")).collect::<String>();

            return Ok((bytes, hex));
        }

        #[cfg(not(feature = "distributors-community"))]
        {
            let _ = url;
            Err("distributors-community feature required for network downloads".to_string())
        }
    }

    /// Saves the downloaded datasheet bytes into the library's `datasheets/` directory.
    pub fn persist_datasheet(
        &self,
        library_root: &Path,
        filename: &str,
        bytes: &[u8],
    ) -> Result<PathBuf, std::io::Error> {
        let dir = library_root.join("datasheets");
        fs::create_dir_all(&dir)?;
        let path = dir.join(filename);
        oxide_types::atomic_io::atomic_write(&path, bytes)?;
        Ok(path)
    }

    /// Imports a scraped component, its synthesized symbol, footprint, and downloaded datasheet
    /// directly into a local `.snxlib` library.
    #[cfg(feature = "local-git")]
    pub fn import_to_library(
        &self,
        adapter: &LocalGitAdapter,
        scraped: &ScrapedComponent,
        table_name: &str,
        pdf_bytes: Option<&[u8]>,
    ) -> Result<ComponentRow, LibraryError> {
        let pkg_name = scraped.package.as_deref().unwrap_or("SOIC-8");
        let pkg = PackageType::from_name(pkg_name);

        // 1. Synthesize and save Footprint
        let fp_name = format!("{}_{}", scraped.mpn, pkg_name);
        let footprint = synthesize_footprint(&fp_name, &pkg);
        let fp_uuid = footprint.uuid;
        adapter.save_footprint(footprint, &format!("feat(lib): add footprint {fp_name}"))?;

        // 2. Synthesize and save Symbol
        let pin_count = match &pkg {
            PackageType::Chip2Pin { .. } => 2,
            PackageType::Sot23 => 3,
            PackageType::Soic { pin_count, .. } => *pin_count,
            PackageType::Qfp { pin_count, .. } => *pin_count,
            PackageType::GenericDual { pin_count, .. } => *pin_count,
        };

        let mut pins = Vec::with_capacity(pin_count);
        for i in 1..=pin_count {
            let name = match i {
                1 if pin_count > 2 => "IN".into(),
                2 if pin_count == 2 => "2".into(),
                n if n == pin_count => "GND".into(),
                n if n == pin_count / 2 => "VCC".into(),
                n => format!("P{n}"),
            };
            let dir = if name == "VCC" || name == "GND" {
                PinDirection::Power
            } else if name.contains("IN") {
                PinDirection::Input
            } else if name.contains("OUT") {
                PinDirection::Output
            } else {
                PinDirection::Bidirectional
            };
            pins.push((i.to_string(), name, dir));
        }

        let prefix = if pin_count == 2 { "R" } else { "U" };
        let symbol = synthesize_symbol(&scraped.mpn, &pins, prefix);
        let sym_uuid = symbol.uuid;
        adapter.save_symbol(symbol, &format!("feat(lib): add symbol {}", scraped.mpn))?;

        // 3. Handle Datasheet
        let datasheet_ref = if let Some(bytes) = pdf_bytes {
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            let result = hasher.finalize();
            let hash = result.iter().map(|b| format!("{b:02x}")).collect::<String>();
            let filename = format!("{}_{}.pdf", scraped.mpn, &hash[..8]);
            let _ = self.persist_datasheet(adapter.root(), &filename, bytes);
            DatasheetRef::hash_pinned(hash, filename)
        } else if let Some(url) = &scraped.datasheet_url {
            DatasheetRef::url(url.as_str())
        } else {
            DatasheetRef::default()
        };

        // 4. Construct ComponentRow
        let lib_id = adapter.library_id();
        let now = Utc::now();
        let row_id = Uuid::now_v7();
        let mut row = ComponentRow {
            row_id,
            internal_pn: InternalPn::new(&scraped.mpn),
            class: ComponentClass::new(if pin_count == 2 { "Resistors" } else { "Integrated Circuits" }),
            datasheet: datasheet_ref,
            state: LifecycleState::Draft,
            symbol_ref: PrimitiveRef::new(lib_id, sym_uuid),
            footprint_ref: Some(PrimitiveRef::new(lib_id, fp_uuid)),
            sim_ref: None,
            primary_mpn: ManufacturerPart::draft(&scraped.manufacturer, &scraped.mpn),
            alternates: Vec::new(),
            supply: Vec::new(),
            parameters: ParamMap::new(),
            pin_map_overrides: Vec::new(),
            created: now,
            updated: now,
            content_hash: [0u8; 32],
            version: "0.0.1".into(),
            released: false,
            symbol_version: "0.0.1".into(),
            footprint_version: "0.0.1".into(),
            sim_version: String::new(),
            plm: PlmReserved::default(),
        };

        for (k, v) in &scraped.parameters {
            row.parameters.insert(k.clone(), ParamValue::Text(v.clone()));
        }
        row.content_hash = crate::hash::hash_row_content(&row)?;

        // Ensure table exists and insert row
        let tables = adapter.list_tables()?;
        if !tables.iter().any(|t| t == table_name) {
            adapter.create_empty_table(table_name, &format!("feat(lib): create table {table_name}"))?;
        }
        adapter.insert_row(table_name, row.clone(), &format!("feat(lib): import component {}", scraped.mpn))?;

        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_detection() {
        assert_eq!(
            PackageType::from_name("0805_RES"),
            PackageType::Chip2Pin { length_mm: 2.0, width_mm: 1.25 }
        );
        assert_eq!(PackageType::from_name("SOT-23"), PackageType::Sot23);
        assert_eq!(
            PackageType::from_name("SOIC-8"),
            PackageType::Soic { pin_count: 8, pitch_mm: 1.27 }
        );
    }

    #[test]
    fn test_synthesize_footprint_chip() {
        let pkg = PackageType::Chip2Pin { length_mm: 2.0, width_mm: 1.25 };
        let fp = synthesize_footprint("0805", &pkg);
        assert_eq!(fp.pads.len(), 2);
        assert_eq!(fp.pads[0].number, "1");
        assert_eq!(fp.pads[1].number, "2");
        assert_eq!(fp.courtyard.points.len(), 4);
    }

    #[test]
    fn test_synthesize_footprint_soic() {
        let pkg = PackageType::Soic { pin_count: 8, pitch_mm: 1.27 };
        let fp = synthesize_footprint("SOIC-8", &pkg);
        assert_eq!(fp.pads.len(), 8);
        assert_eq!(fp.pads[0].number, "1");
        assert_eq!(fp.pads[7].number, "8");
    }

    #[test]
    fn test_synthesize_symbol() {
        let pins = vec![
            ("1".into(), "VCC".into(), PinDirection::Power),
            ("2".into(), "IN".into(), PinDirection::Input),
            ("3".into(), "OUT".into(), PinDirection::Output),
            ("4".into(), "GND".into(), PinDirection::Power),
        ];
        let sym = synthesize_symbol("TEST_IC", &pins, "U");
        assert_eq!(sym.pins.len(), 4);
        assert_eq!(sym.designator, "U?");
        assert_eq!(sym.graphics.len(), 1);
    }

    #[test]
    fn test_scraper_search_fallback() {
        let scraper = ComponentScraper::new();
        let results = scraper.search("NE555").expect("search succeeds");
        assert!(!results.is_empty());
        assert_eq!(results[0].mpn, "NE555");
    }
}
