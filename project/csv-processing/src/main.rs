use csv_processing::read_csv;

fn main() {
    if let Err(err) = read_csv::read_csv_records() {
        eprintln!("Error: {}", err);
    }
}
