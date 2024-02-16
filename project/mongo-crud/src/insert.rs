use mongodb::{options::InsertManyOptions, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Book {
    _id: u64,
    title: String,
    author: String,
}

pub async fn insert_many(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Book> = client.database("sample_restaurants").collection("books");
    let docs = vec![
        Book {
            _id: 1,
            title: "Where the Wild Things Are".to_string(),
            author: "".to_string(),
        },
        Book {
            _id: 2,
            title: "The Very Hungry Caterpillar".to_string(),
            author: "".to_string(),
        },
        Book {
            _id: 1,
            title: "Blueberries for Sal".to_string(),
            author: "".to_string(),
        },
        Book {
            _id: 3,
            title: "Goodnight Moon".to_string(),
            author: "".to_string(),
        },
    ];
    let opts = InsertManyOptions::builder().ordered(false).build();

    let insert_many_result = my_coll.insert_many(docs, opts).await?;
    println!("Inserted documents with _ids:");
    for (_key, value) in &insert_many_result.inserted_ids {
        println!("{:?}", value);
    }

    Ok(())
}
