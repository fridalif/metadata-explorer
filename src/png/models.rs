struct PngChunk {
    pub length: u32,
    pub chunk_type: String,
    pub data: Vec<u8>,
    pub crc: u32
}