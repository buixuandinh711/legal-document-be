use flate2::{write::GzEncoder, Compression};
use std::io::{Result, Write};
use tiny_keccak::Hasher;

pub fn keccak256<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
    let mut output = [0u8; 32];

    let mut hasher = tiny_keccak::Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);

    output
}

pub fn compress(buf: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    
    encoder.write_all(buf)?;
    let compressed_data = encoder.finish()?;

    Ok(compressed_data)
}
