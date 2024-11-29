mod chunk;

use chunk::{PngChunk, HIDE, IEND, IHDR};
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

    // Dumps the PNG file to stdout
    pub fn dump(&mut self) {
        println!("Header: {:08X}", self.header);
        for chunk in self.chunks.iter() {
            chunk.dump();
        }
    }

    // TODO: For the moment we inject a fixed string, make it custom!
    pub fn inject(&mut self, target_png: &str) -> PngResult<()> {
        let mut out = File::create(target_png).map_err(|e| e.to_string())?;
        out.write(&PNG_HEADER).map_err(|e| e.to_string())?;
        for i in 0..self.chunks.len() {
            if self.chunks[i].is_type(IEND) {
                // Let's insert the new chunk just before the end...
                PngChunk::write_custom(&mut out, "Kilroy was here!")?;
            }
            self.chunks[i].write_to_file(&mut self.file, &mut out)?;
        }
        Ok(())
    }

    pub fn extract(&mut self) -> PngResult<()> {
        for chunk in self.chunks.iter() {
            if chunk.is_type(HIDE) {
                chunk.print_data(&mut self.file)?;
                return Ok(());
            }
        }
        Err(String::from("Hidden message not found in this file"))
    }
}
