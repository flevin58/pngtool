mod chunk;

use chunk::{PngChunk, IEND, IHDR};
use std::{
    fs::File,
    io::{Read, Write},
};

type PngResult<T> = Result<T, String>;

const PNG_HEADER: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

pub struct PngFile {
    file: File,
    header: u64,
    chunks: Vec<PngChunk>,
}

impl PngFile {
    pub fn new(filename: &str) -> PngResult<PngFile> {
        let fh = File::open(filename).map_err(|e| e.to_string())?;

        let mut png = PngFile {
            file: fh,
            header: 0,
            chunks: Vec::new(),
        };

        png.read_header()?;

        let mut chunk = PngChunk::new();

        // Read first chunk and add it to the list
        chunk.read_no_data(&mut png.file)?;
        if !chunk.is_type(IHDR) {
            return Err(String::from("Bad chunk: expected IHDR"));
        }
        png.chunks.push(chunk.clone());

        // Read all other chunks until IEND
        while !chunk.is_type(IEND) {
            chunk.read_no_data(&mut png.file)?;
            png.chunks.push(chunk.clone());
        }

        Ok(png)
    }

    fn read_header(&mut self) -> PngResult<()> {
        let mut buffer_u64 = [0_u8; 8];
        self.file
            .read_exact(&mut buffer_u64)
            .map_err(|e| e.to_string())?;
        self.header = u64::from_be_bytes(buffer_u64);
        if buffer_u64 != PNG_HEADER {
            return Err(String::from("Bad header: The file is not a PNG file"));
        }
        Ok(())
    }

    pub fn dump(&mut self) {
        println!("Header: {:08X}", self.header);
        for chunk in self.chunks.iter() {
            chunk.dump();
        }
    }

    // TODO: Add the new chunk to inject after the header
    pub fn inject(&mut self, target_png: &str) -> PngResult<()> {
        let mut out = File::create(target_png).map_err(|e| e.to_string())?;
        out.write(&PNG_HEADER).map_err(|e| e.to_string())?;
        for i in 0..self.chunks.len() {
            // Write data length
            let buffer = self.chunks[i].data_len.to_be_bytes();
            out.write(&buffer).map_err(|e| e.to_string())?;

            // Write data type
            let buffer = self.chunks[i].data_type.to_be_bytes();
            out.write(&buffer).map_err(|e| e.to_string())?;

            // Write data bytes
            self.chunks[i].copy_data(&mut self.file, &mut out)?;

            // Write crc32
            let buffer = self.chunks[i].crc32.to_be_bytes();
            out.write(&buffer).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
