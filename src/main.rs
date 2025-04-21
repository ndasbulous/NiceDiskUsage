mod get_folder_size;

fn main() {
    let results = get_folder_size::get_folder_size("d:\\");

    for entry in &results {
        println!("{} {}", entry.0, entry.1);
    }
}
