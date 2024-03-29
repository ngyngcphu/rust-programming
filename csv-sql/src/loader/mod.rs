use std::cmp::Ordering;
use std::fs::File;

use csv::ReaderBuilder;
use lazy_static::lazy_static;
use regex::Regex;
use rusqlite::Connection;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub c_name: String,
    pub c_category_key: String,
    pub c_category_name: String,
    pub c_source: String,
}

pub fn load_table_from_path(
    db: &mut Connection,
    table_name: &str,
    path: &str,
    delimiter: u8,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let f = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .delimiter(delimiter)
        .from_reader(f);

    let filter_column = reader.headers()?.iter().take(4);

    let normalized_cols = filter_column
        .map(normalize_col)
        .fold(vec![], |mut v, orig_col| {
            let mut col = orig_col.clone();
            let mut i = 1;
            while v.contains(&col) {
                col = format!("{orig_col}_{i}");
                i += 1;
            }
            v.push(col);
            v
        });

    create_table(db, table_name, &normalized_cols);

    let insert_query = format!(
        "insert into {} values ({})",
        table_name,
        normalized_cols
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ")
    );

    let mut records = reader.byte_records();
    let mut num_rows = 0;
    let tx = db.transaction().unwrap();
    {
        let mut stmt = tx.prepare(&insert_query).expect("tx.prepare() failed");
        while let Some(row) = records.next() {
            let mut row = row?;
            row.truncate(normalized_cols.len());

            match row.len().cmp(&normalized_cols.len()) {
                Ordering::Less => {
                    for _ in 0..normalized_cols.len() - row.len() {
                        row.push_field(b"");
                    }
                }
                Ordering::Greater => {
                    panic!("Too many fields on row {}, fields: {:?}", num_rows + 1, row);
                }
                Ordering::Equal => {}
            }
            stmt.execute(rusqlite::params_from_iter(
                row.iter().map(String::from_utf8_lossy),
            ))
            .unwrap();

            num_rows += 1;
        }
    }
    tx.commit().unwrap();

    Ok(normalized_cols)
}

fn normalize_col(col: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\(.*?\)$").unwrap();
    }
    let mut col = RE
        .replace_all(col, "")
        .to_lowercase()
        .trim()
        .replace(['(', ')'], "")
        .replace([' ', '.', '-', '/'], "_")
        .replace('?', "")
        .replace([',', '&'], "_")
        .replace([':', '#'], "");
    if col.chars().next().map(char::is_alphabetic).unwrap_or(false) {
        col = format!("c_{col}");
    }
    col
}

fn create_table(db: &Connection, table_name: &str, cols: &[String]) {
    let create_columns = cols
        .iter()
        .map(|c| format!("\"{c}\" varchar"))
        .collect::<Vec<String>>()
        .join(", ");
    db.execute(
        &format!("create table {table_name} ({create_columns})"),
        &[] as &[&dyn rusqlite::types::ToSql],
    )
    .unwrap();
}

#[cfg(test)]
mod loader_tests {
    use super::*;

    #[test]
    fn test_normalize_col() {
        let mut conn = Connection::open_in_memory().unwrap();
        let col_names = load_table_from_path(
            &mut conn,
            "t",
            "Tracking_Rule_Package_Default_Export.csv",
            b',',
        )
        .unwrap();

        assert_eq!(
            col_names,
            ["c_name", "c_category_key", "c_category_name", "c_source"]
        );
    }

    #[test]
    fn test_create_table() {
        use std::io::Write;

        let mut conn = Connection::open_in_memory().unwrap();
        let _ = load_table_from_path(
            &mut conn,
            "t",
            "Tracking_Rule_Package_Default_Export.csv",
            b',',
        )
        .unwrap();

        let mut stmt = conn.prepare("select * from t").unwrap();

        let records = stmt
            .query_map([], |row| {
                Ok(Record {
                    c_name: row.get(0)?,
                    c_category_key: row.get(1)?,
                    c_category_name: row.get(2)?,
                    c_source: row.get(3)?,
                })
            })
            .unwrap();

        let result = records.map(|item| item.unwrap()).collect::<Vec<_>>();

        let formatted_result = format!("{:#?}", result);
        let mut output = File::create("result.txt").unwrap();
        write!(output, "{}", formatted_result).unwrap();

        assert_eq!(result.len(), 448);
    }
}
