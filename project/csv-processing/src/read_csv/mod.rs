use std::fs::File;
use std::io::BufReader;

use csv::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    name: String,
    category_key: String,
    category_name: String,
    source: String,
    app_name: String,
    url: String,
    keyword: String,
    block_in_break: String,
    block_in_focus: String,
    block_in_meeting: String,
    always_block: String,
    do_not_track_any_urls: bool,
    track_full_url_enabled: bool,
    track_titles_enabled: bool,
}

pub fn read_csv_records() -> Result<(), Error> {
    let input = File::open("Tracking_Rule_Package_Default_Export.csv")?;
    let buffered = BufReader::new(input);
    let mut reader = csv::Reader::from_reader(buffered);
    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "{}, {}, {}, {}.",
            record.name, record.category_name, record.source, record.url
        );
    }

    Ok(())
}
