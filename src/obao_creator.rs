/* uses */
use anyhow::Result;
use bao::{Hash, encode};

// Whatever data we need to keep track of in order to verify challenges.
pub struct ObaoData {
    // The obao file data.
    pub obao: Vec<u8>,
    // The Blake3 hash of the obao file data
    pub hash: Hash, // This is a Bao maintained struct
    // The original size of the file
    pub file_size: u64,
}

impl ObaoData {
    /// Generate a new ObaoData based on bytes.
    /// # Arguments
    /// * 'file_bytes` - the bytes of the file we're creating an ObaoData from.
    ///
    /// # Returns - A new ObaoData.
    pub fn new(file_bytes: Vec<u8>) -> Result<Self> {
        let (obao, hash) = encode::outboard(&file_bytes);
        Ok(Self {
            obao,
            hash,
            file_size: file_bytes.len() as u64,
        })
    }

    // TODO: Implement to_json and from_json
}
