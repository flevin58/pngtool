package png

import (
	"encoding/binary"
	"fmt"
	"os"
)

var PNG_HEADER uint64 = binary.BigEndian.Uint64([]byte{0x89, 'P', 'N', 'G', 0x0D, 0x0A, 0x1A, 0x0A})

type PngFile struct {
	file   *os.File
	header uint64
	chunks []PngChunk
}

func New(path string) (*PngFile, error) {
	fh, err := os.Open(path)
	if err != nil {
		return nil, err
	}

	png := PngFile{
		file:   fh,
		header: 0,
		chunks: []PngChunk{},
	}

	if err := png.readHeader(); err != nil {
		return nil, err
	}
	chunk := PngChunk{}

	// Read first chunk and add it to the list
	chunk.ReadNoData(png.file)
	if chunk.DataType != IHDR {
		return nil, fmt.Errorf("bad chunk: expected IHDR")
	}
	png.chunks = append(png.chunks, chunk)

	// Read all other chunks until IEND
	for chunk.DataType != IEND {
		chunk.ReadNoData(png.file)
		png.chunks = append(png.chunks, chunk)
	}
	return &png, nil
}

func (pf *PngFile) readHeader() error {
	binary.Read(pf.file, binary.BigEndian, &pf.header)
	if pf.header != PNG_HEADER {
		return fmt.Errorf("bad header: The file is not a PNG file")
	}
	return nil
}

func (pf *PngFile) SecretMessage() string {
	for _, chunk := range pf.chunks {
		if chunk.DataType == HIDE {
			buf := make([]byte, chunk.DataLen)
			pf.file.ReadAt(buf, chunk.DataPtr)
			return string(buf)
		}
	}
	return ""
}

// Dumps the PNG file to stdout
func (pf *PngFile) Dump() {
	fmt.Printf("Header: %08X\n", pf.header)
	var lastType ChunkType = NONE
	var count uint32 = 1
	var totalLen uint32 = 0
	for _, chunk := range pf.chunks {
		if lastType != chunk.DataType {
			if count > 1 {
				fmt.Printf("  repeated: %d time(s)\n", count)
				fmt.Printf("  total length: %d\n", totalLen)
			}
			chunk.dump()
			lastType = chunk.DataType
			count = 0
			totalLen = 0
		}
		count++
		totalLen += chunk.DataLen
	}
}

func (pf *PngFile) Inject(target string, msg string) error {
	out, err := os.Create(target)
	if err != nil {
		return err
	}

	if err = binary.Write(out, binary.BigEndian, PNG_HEADER); err != nil {
		return err
	}

	for _, chunk := range pf.chunks {
		if chunk.DataType == IEND {
			// Let's inject the message just before the end...
			writeCustom(out, msg)
		}
		chunk.writeToFile(pf.file, out)
	}
	return nil
}
