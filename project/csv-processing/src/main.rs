//use csv_processing::filter_csv;
//use csv_processing::delimiter;
//use csv_processing::read_csv;
//use csv_processing::serialize_csv;
use csv_processing::transform_csv;

fn main() {
    // if let Err(err) = read_csv::read_csv_records() {
    //     eprintln!("Error: {}", err);
    // }

    // if let Err(err) = delimiter::read_csv_with_different_delimiter() {
    //     eprintln!("Error: {}", err);
    // }

    // if let Err(err) = filter_csv::filter_csv_matching_predicate() {
    //     eprintln!("Error: {}", err);
    // }

    // if let Err(err) = serialize_csv::serialize_records_to_csv() {
    //     eprintln!("Error: {}", err);
    // }

    if let Err(err) = transform_csv::transform_csv_column() {
        eprintln!("Error: {}", err);
    }
}
