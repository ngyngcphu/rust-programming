//use file_system::read_line;
use file_system::same_file;

fn main() {
    // if let Err(err) = read_line::read_lines_of_strings_from_file() {
    //     eprintln!("Error: {}", err);
    // }
    if let Err(err) = same_file::write_and_read_from_file() {
        eprintln!("Error: {}", err);
    }
}
