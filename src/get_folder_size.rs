use fs_extra::dir::get_size;
use std::fs;

pub fn get_folder_size(path_to_read: &str) -> Vec<(String, u64)> {
    let paths = fs::read_dir(path_to_read).unwrap();

    let mut results: Vec<(String, u64)> = Vec::new();

    let mut counter: i32 = 0;
    for path in paths {
        counter += 1;
        // Unwrap directory entry
        let entry = match path {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
                continue;
            }
        };

        let path = entry.path();

        // Get folder size
        let folder_size = match get_size(&path) {
            Ok(size) => size,
            Err(e) => {
                eprintln!("Error getting folder size for {:?}: {}", path, e);
                continue;
            }
        };

        // Convert file_name to a string
        match entry.file_name().into_string() {
            Ok(file_name) => {
                // Print folder name and size
                results.push((file_name, folder_size));
            }
            Err(_) => {
                eprintln!("Error converting file name to string");
            }
        }
    }

    results
}
