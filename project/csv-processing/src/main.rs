use csv_processing::filter_csv;
use csv_processing::delimiter;
//use csv_processing::read_csv;

fn main() {
    // if let Err(err) = read_csv::read_csv_records() {
    //     eprintln!("Error: {}", err);
    // }

    if let Err(err) = delimiter::read_csv_with_different_delimiter() {
        eprintln!("Error: {}", err);
    }

    if let Err(err) = filter_csv::filter_csv_matching_predicate() {
        eprintln!("Error: {}", err);
    }
}
