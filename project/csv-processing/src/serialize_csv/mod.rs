use std::io;

use csv::Result;
use serde::Serialize;

#[derive(Serialize)]
struct Record<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}

pub fn serialize_records_to_csv() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let rec1 = Record {
        name: "Mark",
        place: "Melbourne",
        id: 56,
    };
    let rec2 = Record {
        name: "Ashley",
        place: "Sydney",
        id: 64,
    };
    let rec3 = Record {
        name: "Akshat",
        place: "Delhi",
        id: 98,
    };

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;
    Ok(())
}
