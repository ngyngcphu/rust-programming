use std::{fs, path};

use csv_sql::{loader, utils};
use rusqlite::Connection;

fn main() {
    let db_path = format!("{}/db.sqlite3", utils::get_data_directory());
    if !path::Path::new(&db_path).exists() {
        if let Some(parent_dir) = path::Path::new(&db_path).parent() {
            fs::create_dir_all(parent_dir).expect("Failed to create directory");
        }
    }
    fs::File::create(&db_path).expect("Failed to create file");
    let mut conn = Connection::open(db_path).unwrap();
    loader::load_table_from_path(
        &mut conn,
        "t_tracking_rule",
        "Tracking_Rule_Package_Default_Export.csv",
        b',',
    )
    .unwrap();

    let mut stmt = conn.prepare("select * from t_tracking_rule").unwrap();
    let records = stmt.query_map([], |row| {
        Ok(loader::Record {
            c_name: row.get(0)?,
            c_category_key: row.get(1)?,
            c_category_name: row.get(2)?,
            c_source: row.get(3)?,
        })
    }).unwrap();
    for record in records {
        println!("{:#?}", record.unwrap());
    }
}
