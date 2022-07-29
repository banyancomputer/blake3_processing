use anyhow::{anyhow, Error, Result};
use rand::Rng;
use std::convert::TryInto;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};

// How big File chunks are with Bao
// TODO: Subject to change, we need to coordinate with bao team.
pub const BAO_CHUNK_SIZE: usize = 1024;

struct FakeSeeker<R: Read> {
    reader: R,
    bytes_read: u64,
}

impl<R: Read> FakeSeeker<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            bytes_read: 0,
        }
    }
}

impl<R: Read> Read for FakeSeeker<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.reader.read(buf)?;
        self.bytes_read += n as u64;
        Ok(n)
    }
}

impl<R: Read> Seek for FakeSeeker<R> {
    fn seek(&mut self, _: SeekFrom) -> std::io::Result<u64> {
        // Do nothing and return the current position.
        Ok(self.bytes_read)
    }
}

pub struct ObaoSlice {
    pub slice: Vec<u8>,
    pub start_index: usize,
}

// Our Implementation for creating and verifying ObaoSlices from file chunks.
impl ObaoSlice {
    /// Generate a new ObaoSlice from a file.
    /// # Arguments
    /// * 'obao' - The bytes of the obao file to use to generate the ObaoSlice.
    /// * 'file_chunk' - The file chunk to use to generate the ObaoSlice. These chunk must be of size BAO_CHUNK_SIZE.
    ///                  Note that these chunks aligned with the obao file, and don't pass Chunk boundaries!
    /// * 'start_byte' - The byte offset of the first chunk from the file.
    /// Returns: A new ObaoSlice.
    pub fn new(obao: Vec<u8>, chunk: &[u8], start_index: usize) -> Result<ObaoSlice, Error> {
        // Check that the chunk is of the correct size.
        if chunk.len() != BAO_CHUNK_SIZE {
            return Err(anyhow!("Chunk is not of size {}", BAO_CHUNK_SIZE));
        }

        // Declare a Vector to hold our slice.
        let mut slice = Vec::new();
        // Extract the slice from the obao file.
        let mut slice_extractor = bao::encode::SliceExtractor::new_outboard(
            FakeSeeker::new(chunk),
            Cursor::new(&obao[..]),
            start_index as u64,
            BAO_CHUNK_SIZE.try_into().unwrap(),
        );
        // Extract the slice.
        slice_extractor.read_to_end(&mut slice)?;
        // Return the ObaoSlice.
        Ok(ObaoSlice { slice, start_index })
    }

    /// Verify the ObaoSlice against the file chunk.
    /// # Arguments
    /// * `hash` - The Blake3 hash of the file chunk.
    /// Returns: A bool indicating if the ObaoSlice is valid.
    pub fn verify(&self, hash: &bao::Hash) -> Result<bool> {
        // Declare a Vector to hold our decoding of the ObaoSlice.
        let mut decoded = Vec::new();

        // Decode the ObaoSlice.
        let mut decoder = bao::decode::SliceDecoder::new(
            &*self.slice,
            hash,
            self.start_index as u64,
            BAO_CHUNK_SIZE.try_into().unwrap(),
        );

        // Read the decoded ObaoSlice into the decoded Vector.
        match decoder.read_to_end(&mut decoded) {
            Err(_) => Ok(false),
            _ => Ok(true),
        }
    }

    /// Decode the ObaoSlice into a file chunk.
    /// # Arguments
    /// * `hash` - The Blake3 hash of the file chunk.
    /// Returns: A file chunk.
    pub fn decode(&self, hash: &bao::Hash) -> Result<Vec<u8>, Error> {
        // Declare a Vector to hold our decoding of the ObaoSlice.
        let mut decoded = Vec::new();

        // Decode the ObaoSlice.
        let mut decoder = bao::decode::SliceDecoder::new(
            &*self.slice,
            hash,
            self.start_index as u64,
            BAO_CHUNK_SIZE.try_into().unwrap(),
        );

        // Read the decoded ObaoSlice into the decoded Vector.
        match decoder.read_to_end(&mut decoded) {
            Err(_) => Err(anyhow!("ObaoSlice is invalid")),
            _ => Ok(decoded),
        }
    }
}

// Generate a random chunk index for a file of size `file_size`.
pub fn generate_random_chunk_index(file_size: usize) -> usize {
    let range = file_size / BAO_CHUNK_SIZE;
    let start_index = rand::thread_rng().gen_range(0..range) * BAO_CHUNK_SIZE;

    // Return the index of the chunk.s
    start_index as usize
}
