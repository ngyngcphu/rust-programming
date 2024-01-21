use csv::{Error, ReaderBuilder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

pub fn read_csv_with_different_delimiter() -> Result<(), Error> {
    let data = "name\tplace\tid
mark\tsydney\t46.5
Mark\tMelbourne\t46
Ashley\tZurich\t92
alisha\tcolombo\txyz";
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    for result in reader.deserialize::<Record>() {
        println!("{:?}", result?);
    }

    Ok(())
}
