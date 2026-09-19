//! Pure-Rust Compound File Binary (CFB / OLE2) parser and stream extractor.

use std::collections::HashMap;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;

use crate::error::AltiumImportError;

const CFB_MAGIC: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
const ENDOFCHAIN: u32 = 0xFFFFFFFE;
const FREESECT: u32 = 0xFFFFFFFF;

/// A Compound File Binary (OLE2) container holding named streams and storages.
#[derive(Debug, Clone)]
pub struct CfbContainer {
    pub streams: HashMap<String, Vec<u8>>,
}

impl CfbContainer {
    /// Parse a Compound File Binary container from a raw byte slice.
    pub fn parse(data: &[u8]) -> Result<Self, AltiumImportError> {
        if data.len() < 512 || data[0..8] != CFB_MAGIC {
            return Err(AltiumImportError::InvalidCfbHeader);
        }

        let sector_shift = (&data[30..32]).read_u16::<LittleEndian>()? as usize;
        let sector_size = 1 << sector_shift;
        if sector_size == 0 || sector_size > 65536 {
            return Err(AltiumImportError::InvalidCfbHeader);
        }

        let num_fat_sectors = (&data[44..48]).read_u32::<LittleEndian>()? as usize;
        let first_dir_sector = (&data[48..52]).read_u32::<LittleEndian>()?;
        let _mini_cutoff = (&data[56..60]).read_u32::<LittleEndian>()?;
        let first_mini_fat = (&data[60..64]).read_u32::<LittleEndian>()?;
        let _num_mini_fat = (&data[64..68]).read_u32::<LittleEndian>()?;

        // 1. Read DIFAT entries (first 109 entries are in the header)
        let mut fat_sector_locs = Vec::with_capacity(num_fat_sectors);
        let mut rdr = &data[76..512];
        for _ in 0..109 {
            if fat_sector_locs.len() >= num_fat_sectors {
                break;
            }
            let sec = rdr.read_u32::<LittleEndian>()?;
            if sec != FREESECT && sec != ENDOFCHAIN {
                fat_sector_locs.push(sec);
            }
        }

        // 2. Build the FAT table
        let u32_per_sector = sector_size / 4;
        let mut fat = Vec::new();
        for &sec_id in &fat_sector_locs {
            let offset = (sec_id as usize + 1) * sector_size;
            if offset + sector_size <= data.len() {
                let mut sec_rdr = &data[offset..offset + sector_size];
                for _ in 0..u32_per_sector {
                    fat.push(sec_rdr.read_u32::<LittleEndian>()?);
                }
            }
        }

        // Helper to read a sector chain from standard FAT
        let read_chain = |start_sec: u32, size: usize| -> Vec<u8> {
            let mut result = Vec::with_capacity(size);
            let mut curr = start_sec;
            let mut visited = 0;
            while curr != ENDOFCHAIN && curr != FREESECT && (curr as usize) < fat.len() {
                let offset = (curr as usize + 1) * sector_size;
                if offset >= data.len() {
                    break;
                }
                let end = (offset + sector_size).min(data.len());
                result.extend_from_slice(&data[offset..end]);
                curr = fat[curr as usize];
                visited += 1;
                if visited > fat.len() || result.len() >= size + sector_size {
                    break;
                }
            }
            if result.len() > size {
                result.truncate(size);
            }
            result
        };

        // 3. Read Directory stream
        let dir_data = read_chain(first_dir_sector, 1024 * 1024);
        let mut streams = HashMap::new();

        // 4. Parse 128-byte Directory Entries
        let num_entries = dir_data.len() / 128;
        let mut root_start_sec = 0u32;
        let mut root_size = 0usize;

        // Parse Root Storage first (entry 0)
        if num_entries > 0 {
            let entry = &dir_data[0..128];
            root_start_sec = (&entry[116..120]).read_u32::<LittleEndian>()?;
            root_size = (&entry[120..128]).read_u64::<LittleEndian>()? as usize;
        }

        let mini_stream = if root_start_sec != ENDOFCHAIN && root_size > 0 {
            read_chain(root_start_sec, root_size)
        } else {
            Vec::new()
        };

        // Parse MiniFAT if present
        let mini_fat = if first_mini_fat != ENDOFCHAIN {
            let mini_fat_bytes = read_chain(first_mini_fat, 65536);
            let mut mf = Vec::with_capacity(mini_fat_bytes.len() / 4);
            let mut mrdr = &mini_fat_bytes[..];
            while mrdr.len() >= 4 {
                mf.push(mrdr.read_u32::<LittleEndian>()?);
            }
            mf
        } else {
            Vec::new()
        };

        for i in 1..num_entries {
            let entry = &dir_data[i * 128..(i + 1) * 128];
            let name_len = (&entry[64..66]).read_u16::<LittleEndian>()? as usize;
            let obj_type = entry[66];
            let start_sec = (&entry[116..120]).read_u32::<LittleEndian>()?;
            let stream_size = (&entry[120..128]).read_u64::<LittleEndian>()? as usize;

            if obj_type == 2 && name_len > 2 {
                // Stream object
                let name_bytes = &entry[0..name_len.min(64)];
                let utf16_chars: Vec<u16> = name_bytes
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .take_while(|&c| c != 0)
                    .collect();
                let stream_name = String::from_utf16_lossy(&utf16_chars);

                let stream_content = if stream_size < 4096 && !mini_stream.is_empty() && !mini_fat.is_empty() {
                    // Mini stream reading (64 byte mini sectors)
                    let mini_sector_size = 64;
                    let mut result = Vec::with_capacity(stream_size);
                    let mut mcurr = start_sec;
                    let mut mvisited = 0;
                    while mcurr != ENDOFCHAIN && mcurr != FREESECT && (mcurr as usize) < mini_fat.len() {
                        let offset = mcurr as usize * mini_sector_size;
                        if offset >= mini_stream.len() {
                            break;
                        }
                        let end = (offset + mini_sector_size).min(mini_stream.len());
                        result.extend_from_slice(&mini_stream[offset..end]);
                        mcurr = mini_fat[mcurr as usize];
                        mvisited += 1;
                        if mvisited > mini_fat.len() || result.len() >= stream_size + mini_sector_size {
                            break;
                        }
                    }
                    if result.len() > stream_size {
                        result.truncate(stream_size);
                    }
                    result
                } else {
                    read_chain(start_sec, stream_size)
                };

                streams.insert(stream_name, stream_content);
            }
        }

        Ok(Self { streams })
    }

    /// Read and optionally decompress a named stream.
    pub fn get_stream(&self, name: &str) -> Result<Vec<u8>, AltiumImportError> {
        let raw = self
            .streams
            .get(name)
            .ok_or_else(|| AltiumImportError::StreamNotFound(name.to_string()))?;

        // Check if stream is zlib compressed (typical zlib header: 0x78 0x9C or 0x78 0x01 or 0x78 0xDA)
        if raw.len() > 2 && raw[0] == 0x78 && (raw[1] == 0x9C || raw[1] == 0x01 || raw[1] == 0xDA || raw[1] == 0x5E) {
            let mut decoder = ZlibDecoder::new(&raw[..]);
            let mut decompressed = Vec::new();
            if decoder.read_to_end(&mut decompressed).is_ok() && !decompressed.is_empty() {
                return Ok(decompressed);
            }
        }

        Ok(raw.clone())
    }
}
