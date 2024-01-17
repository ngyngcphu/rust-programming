use file_system::read_line;

fn main() {
    if let Err(err) = read_line::read_lines_of_strings_from_file() {
        eprintln!("Error: {}", err);
    }
}
