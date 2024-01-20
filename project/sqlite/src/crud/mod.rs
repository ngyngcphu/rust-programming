use std::collections::HashMap;

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

pub fn create_database() -> Result<()> {
    let conn = Connection::open("cats.db")?;
    conn.execute(
        "create table if not exists cat_colors (
                id integer primary key,
                name text not null unique
            )",
        [],
    )?;
    conn.execute(
        "create table if not exists cats (
                id integer primary key,
                name text not null,
                color_id integer not null references cat_colors(id) on delete cascade
            )",
        [],
    )?;

    Ok(())
}

pub fn insert_data() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in cat_colors {
        conn.execute(
            "insert into cat_colors (name) values (?1)",
            [color.to_string()],
        )?;
        let last_id = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "insert into cats (name, color_id) values (?1, ?2)",
                [&cat.to_string(), &last_id],
            )?;
        }
    }

    Ok(())
}

pub fn select_data() -> Result<()> {
    let conn = Connection::open("cats.db")?;
    let mut stmt = conn.prepare(
        "select c.name, cc.name from cats c
            inner join cat_colors cc
            on cc.id = c.color_id;",
    )?;
    let cats = stmt.query_map([], |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;
    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}
