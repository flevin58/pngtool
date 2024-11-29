pub const IHDR: u32 = 0x49484452;
pub const IEND: u32 = 0x49454E44;
pub const HIDE: u32 = u32::from_be_bytes([b'h', b'I', b'D', b'e']);

use crc32fast::Hasher;
use std::{
    fmt,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use super::PngResult;

type ChunkResult<T> = Result<T, String>;

// The PNG file consists of a header + a number of chunks
#[derive(Default, Debug, Clone, Copy)]
pub struct PngChunk {
    pub data_len: u32,  // The length of the data
    pub data_type: u32, // The chunk type (IHDR, IEND, etc.)
    pub data_ptr: u64,  // Pointer to the file position where the data is located
    pub crc32: u32,     // The crc32 of the type + actual data bytes
}

// Trait to allow println!("{}", chunk) giving the ascii type as output (i.e. 'IHDR', etc.)
impl fmt::Display for PngChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = self.data_type.to_be_bytes();
        write!(
            f,
            "{}{}{}{}",
            t[0] as char, t[1] as char, t[2] as char, t[3] as char
        )
    }
}

impl PngChunk {
    // Initializes an empty chunk
    pub fn new() -> PngChunk {
        PngChunk {
            data_len: 0,
            data_type: 0,
            data_ptr: 0,
            crc32: 0,
        }
    }

    // Initializes a custom chunk for 'hidden' data named ... well ... "hiDE"
    // For the moment we can embed only plain text (TODO: embed files ... zip ... encrypt ???)
    pub fn write_custom(fout: &mut File, text: &str) -> PngResult<()> {
        let mut hasher = Hasher::new();
        hasher.update(&u32::to_be_bytes(HIDE));
        hasher.update(text.as_bytes());

        let chunk = PngChunk {
            data_len: text.len() as u32,
            data_type: HIDE,
            data_ptr: 0,
            crc32: hasher.finalize(),
        };

        // Write data length
        let buffer = chunk.data_len.to_be_bytes();
        fout.write(&buffer).map_err(|e| e.to_string())?;

        // Write data type
        let buffer = chunk.data_type.to_be_bytes();
        fout.write(&buffer).map_err(|e| e.to_string())?;

        // Write data bytes
        fout.write(text.as_bytes()).map_err(|e| e.to_string())?;

        // Write crc32
        let buffer = chunk.crc32.to_be_bytes();
        fout.write(&buffer).map_err(|e| e.to_string())?;

        Ok(())
    }

    // Checks if the chunk type is equal to the given argument
    pub fn is_type(&self, dtype: u32) -> bool {
        return self.data_type == dtype;
    }

    pub fn print_data(&self, fin: &mut File) -> ChunkResult<()> {
        fin.seek(SeekFrom::Start(self.data_ptr))
            .map_err(|e| e.to_string())?;

        let mut buf: Vec<u8> = vec![0; self.data_len as usize];
        fin.read(&mut buf).map_err(|e| e.to_string())?;

        println!("Found secret message:");
        println!("{}", String::from_utf8(buf).map_err(|e| e.to_string())?);
        Ok(())
    }

    // Dumps the chunk contents (excluding the data) to stdout
    pub fn dump(&self) {
        println!("Chunk: {} (0x{:04X})", self, self.data_type);
        println!("  length: {} bytes", self.data_len);
        println!("  crc32:  {:04X}", self.crc32);
    }

    // Fills itself with data read from the file at the current location.
    // Note that to save memory, no data is actually stored, just the file pointer.
    pub fn read_no_data(&mut self, file: &mut File) -> ChunkResult<()> {
        let mut buffer_u32 = [0_u8; 4];
        let mut hasher = Hasher::new();

        // Read length
        file.read_exact(&mut buffer_u32)
            .map_err(|e| e.to_string())?;
        self.data_len = u32::from_be_bytes(buffer_u32);

        // Read type and update crc32
        file.read_exact(&mut buffer_u32)
            .map_err(|e| e.to_string())?;
        hasher.update(&buffer_u32);
        self.data_type = u32::from_be_bytes(buffer_u32);

        // Here we point at the actual data. Save the pointer.
        self.data_ptr = file.stream_position().map_err(|e| e.to_string())?;

        // Skip data but calculate crc32
        let mut char_buf = [0; 1];
        for _ in 0..self.data_len {
            file.read(&mut char_buf).map_err(|e| e.to_string())?;
            hasher.update(&char_buf);
        }

        // Read the crc32 from the file and check it
        file.read_exact(&mut buffer_u32)
            .map_err(|e| e.to_string())?;
        self.crc32 = u32::from_be_bytes(buffer_u32);
        if hasher.finalize() != self.crc32 {
            return Err(String::from("Bad crc32 in chunk"));
        }

        Ok(())
    }

    pub fn write_to_file(&self, fin: &mut File, fout: &mut File) -> ChunkResult<()> {
        // Write data length
        let buffer = self.data_len.to_be_bytes();
        fout.write(&buffer).map_err(|e| e.to_string())?;

        // Write data type
        let buffer = self.data_type.to_be_bytes();
        fout.write(&buffer).map_err(|e| e.to_string())?;

        // Write data bytes
        self.copy_data(fin, fout)?;

        // Write crc32
        let buffer = self.crc32.to_be_bytes();
        fout.write(&buffer).map_err(|e| e.to_string())?;

        Ok(())
    }

    // Copies iself from
    pub fn copy_data(&self, fin: &mut File, fout: &mut File) -> ChunkResult<()> {
        let mut how_many = self.data_len;
        const CHUNK_BUF_CAP: u32 = 32 * 1024;

        fin.seek(SeekFrom::Start(self.data_ptr))
            .map_err(|e| e.to_string())?;

        // eprintln!("Writing {} bytes from {}", how_many, self.data_ptr);

        while how_many > 0 {
            let mut cap = how_many;
            if cap > CHUNK_BUF_CAP {
                cap = CHUNK_BUF_CAP;
            }
            let mut buf: Vec<u8> = vec![0; cap as usize];
            fin.read(&mut buf).map_err(|e| e.to_string())?;
            fout.write(&buf).map_err(|e| e.to_string())?;
            how_many -= cap;
        }

        Ok(())
    }
}
