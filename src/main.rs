mod get_folder_size;

fn main() {
    let results = get_folder_size::get_folder_size("d:\\");

    for entry in &results {
        print!("{} ", entry.0);
        match entry.1 {
            0..1024 => {
                println!("{} B", entry.1);
            }
            1024..1048576 => {
                println!("{} KB", entry.1 / 1024);
            }
            1048576..1073741824 => {
                println!("{} MB", entry.1 / 1024 / 1024);
            }
            _ => {
                println!("{} GB", entry.1 / 1024 / 1024 / 1024);
            }
        }
    }
}
