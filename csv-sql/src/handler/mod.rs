use rusqlite::types::{FromSql, FromSqlError, ToSql, ValueRef};
use rusqlite::{Connection, Statement};

struct FromAnySqlType {
    value: String,
}

impl FromSql for FromAnySqlType {
    fn column_result(value: ValueRef<'_>) -> Result<FromAnySqlType, FromSqlError> {
        let result = match value {
            ValueRef::Null => "null".to_string(),
            ValueRef::Integer(v) => v.to_string(),
            ValueRef::Real(v) => v.to_string(),
            ValueRef::Blob(v) | ValueRef::Text(v) => String::from_utf8(v.to_vec()).unwrap(),
        };

        Ok(FromAnySqlType { value: result })
    }
}

fn handle_query(conn: &Connection, line: &str) -> Result<(), String> {
    let mut stmt = prepare_query(conn, line)?;
    let col_count = stmt.column_count();

    let mut results = stmt.query(&[] as &[&dyn ToSql]).unwrap();
    while let Ok(Some(r)) = results.next() {
        for i in 0..col_count {
            let cell: FromAnySqlType = r.get(i).unwrap();
        }
    }

    Ok(())
}

fn prepare_query<'a>(conn: &'a Connection, query: &str) -> Result<Statement<'a>, String> {
    conn.prepare(query).map_err(|e| e.to_string())
}
