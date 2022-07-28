// Define out library

extern crate anyhow;

pub mod obao_creator;
pub mod obao_verifier;

// use serde_json;
use serde::{Serialize};

// The data associated with a single slice of a file.
#[derive(Serialize)]
struct SliceData {
    pub slice_index: usize, // The index of the slice.
    pub slice_name: String, // The name of the slice.
    pub size: usize, // How big the slice is.
    pub verified: bool, // Whether the slice has been verified.
}

// The data associated with a file.
#[derive(Serialize)]
struct TestData {
    pub hash: String,
    // The hash of the file.
    pub size: usize,
    // The length of the file.
    pub blocks: usize,
    // The number of Bao blocks in the file.
    pub slices: Vec<SliceData>, // The slices of the file.
}

#[cfg(test)]
mod test {
    /* uses */
    use super::*;

    // Iterate through all the files in the test directory and generate an ObaoData for each.
    // Save obao data in tests/obaos/<filename>.obao.
    // Then iterate through every chunk of the file and generate a slice of the ObaoData.
    // Save every chunk in tests/slices/<filename>/<slice_index>
    // Compile results into a json file in tests/results/<filename>.json.
    #[test]
    fn run_tests() {
        // Iterate through every file
        for entry in std::fs::read_dir("tests/files").unwrap() {
            // Declare a new TestData struct for this file.
            let mut file_data = TestData{
                hash: String::new(),
                size: 0,
                blocks: 0,
                slices: Vec::new(),
            };

            let entry = entry.unwrap();
            let path = entry.path();

            // The name of the file we're testing.
            let file_name = path.file_name().unwrap().to_str().unwrap();
            // The bytes of the file we're testing.
            let file_bytes = std::fs::read(&path).unwrap();

            // Create a new ObaoData for the file.
            let file_obao_data = obao_creator::ObaoData::new(&file_bytes).unwrap();
            // Save the ObaoData to a file.
            std::fs::write(
                format!("tests/obaos/{}.obao", file_name),
                &file_obao_data.obao
            ).unwrap();

            // Extract info from the ObaoData.
            file_data.hash = file_obao_data.hash.to_string();
            file_data.size = file_obao_data.file_size;
            file_data.blocks = file_data.size / obao_verifier::BAO_CHUNK_SIZE;

            // Make a directory for the slices of this file.
            // Check if the directory exists. If it doesn't, create it.
            let slice_dir = format!("tests/slices/{}", file_name);
            if !std::path::Path::new(&slice_dir).exists() {
                std::fs::create_dir(&slice_dir).unwrap();
            }

            for i in 0..file_data.blocks {
                // The name of the slice.
                let slice_name = format!("{}/{}", file_name, i);
                // The bytes of the chunk from the file.
                let chunk = &file_bytes[i * obao_verifier::BAO_CHUNK_SIZE..(i + 1) * obao_verifier::BAO_CHUNK_SIZE];

                // The actual obao slice for the chunk.
                let obao_slice = obao_verifier::ObaoSlice::new(
                    file_obao_data.obao.clone(), chunk, i * obao_verifier::BAO_CHUNK_SIZE
                ).unwrap();

                // Verify the slice.
                let verified = obao_slice.verify(&file_obao_data.hash).unwrap();
                // The length of the slice.
                let size = obao_slice.slice.len();

                // Save the obao slice to a file.
                std::fs::write(
                    format!("tests/slices/{}/{}", file_name, i),
                    obao_slice.slice
                ).unwrap();

                let slice_data = SliceData {
                    slice_index: i,
                    slice_name: slice_name.clone(),
                    size,
                    verified,
                };

                // Assert that the slice is verified.
                assert!(verified, "Slice {} is not verified.", &slice_name);
                // Assert that the slice is less than 2kb.
                assert!(size < 2048, "Slice {} is too large: {} b", &slice_name, size);

                file_data.slices.push(slice_data);
            }
            // Save file_data to a json file.
            let json = serde_json::to_string_pretty(&file_data).unwrap();
            std::fs::write(
                format!("tests/results/{}.json", file_name),
                json
            ).unwrap();
        }
    }
}
