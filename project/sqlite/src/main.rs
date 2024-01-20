use sqlite::{crud, transaction};

fn main() {
    if let Err(err) = crud::create_database() {
        eprintln!("Error: {}", err);
    }

    if let Err(err) = crud::insert_data() {
        eprintln!("Error: {}", err);
    }

    if let Err(err) = crud::select_data() {
        eprintln!("Error: {}", err);
    }

    if let Err(err) = transaction::excute_transaction() {
        eprintln!("Error: {}", err);
    }
}
