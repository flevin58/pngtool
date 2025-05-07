package png

import (
	"encoding/binary"
	"fmt"
	"hash/crc32"
	"io"
)

type ChunkType uint32

const NONE ChunkType = 0x00000000
const IHDR ChunkType = 0x49484452
const IEND ChunkType = 0x49454E44
const HIDE ChunkType = 0x68494465

func (ct ChunkType) AsBytes() []byte {
	buffer := make([]byte, 4)
	binary.BigEndian.PutUint32(buffer, uint32(ct))
	return buffer
}

// The PNG file consists of a header + a number of chunks
type PngChunk struct {
	DataLen  uint32    // The length of the data
	DataType ChunkType // The chunk type (IHDR, IEND, etc.)
	DataPtr  int64     // Pointer to the file position where the data is located
	Crc32    uint32    // The crc32 of the type + actual data bytes
}

// Trait to allow println!("{}", chunk) giving the ascii type as output (i.e. 'IHDR', etc.)
func (pc *PngChunk) String() string {
	return string(pc.DataType.AsBytes())
}

// Initializes a custom chunk for 'hidden' data named ... well ... "hiDE"
// For the moment we can embed only plain text (TODO: embed files ... zip ... encrypt ???)
func writeCustom(writer io.Writer, text string) {
	hasher := crc32.NewIEEE()
	hasher.Write(HIDE.AsBytes())
	hasher.Write([]byte(text))

	binary.Write(writer, binary.BigEndian, uint32(len(text)))
	binary.Write(writer, binary.BigEndian, HIDE)
	binary.Write(writer, binary.BigEndian, []byte(text))
	binary.Write(writer, binary.BigEndian, hasher.Sum32())
}

// func (pc *PngChunk) printData(reader io.ReadSeeker) error {
// 	if _, err := reader.Seek(int64(pc.DataPtr), io.SeekStart); err != nil {
// 		return err
// 	}
// 	buffer := make([]byte, pc.DataLen)
// 	if _, err := reader.Read(buffer); err != nil {
// 		return err
// 	}

// 	// Check if the chunk is a custom chunk
// 	fmt.Printf("Found secret message:\n%s\n", string(buffer))
// 	return nil
// }

// Dumps the chunk contents (excluding the data) to stdout
func (pc *PngChunk) dump() {
	fmt.Printf("Chunk: %s (0x%04X), Length: %d, Pos: %d\n", pc, pc.DataType, pc.DataLen, pc.DataPtr)
}

// Fills itself with data read from the file at the current location.
// Note that to save memory, no data is actually stored, just the file pointer.
func (pc *PngChunk) ReadNoData(reader io.ReadSeeker) error {
	hasher := crc32.NewIEEE()
	binary.Read(reader, binary.BigEndian, &pc.DataLen)
	binary.Read(reader, binary.BigEndian, &pc.DataType)
	hasher.Write(pc.DataType.AsBytes())
	pc.DataPtr, _ = reader.Seek(0, io.SeekCurrent)

	// Skip data but calculate crc32
	var char_buf [1]byte
	for range pc.DataLen {
		reader.Read(char_buf[:])
		hasher.Write(char_buf[:])
	}

	// Read the crc32 from the file and check it
	binary.Read(reader, binary.BigEndian, &pc.Crc32)
	if hasher.Sum32() != pc.Crc32 {
		return fmt.Errorf("bad crc32 in chunk")
	}
	return nil
}

func (pc *PngChunk) writeToFile(reader io.ReadSeeker, writer io.Writer) error {
	var err error

	if err = binary.Write(writer, binary.BigEndian, pc.DataLen); err != nil {
		return err
	}
	if err = binary.Write(writer, binary.BigEndian, pc.DataType); err != nil {
		return err
	}

	// Write data bytes
	if err = pc.copyData(reader, writer); err != nil {
		return err
	}

	// Write crc32
	if err = binary.Write(writer, binary.BigEndian, pc.Crc32); err != nil {
		return err
	}
	return nil
}

func (pc *PngChunk) copyData(reader io.ReadSeeker, writer io.Writer) error {
	reader.Seek(pc.DataPtr, io.SeekStart)
	len := pc.DataLen
	buf := make([]byte, 1)
	for len > 0 {
		if _, err := reader.Read(buf); err != nil {
			return err
		}
		writer.Write(buf)
		len--
	}
	return nil
}
