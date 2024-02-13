use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Restaurant {
    borough: Option<String>,
    cuisine: String,
    name: String,
}

pub async fn insert_one(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let doc = Restaurant {
        name: "Sea Stone Tavern".to_string(),
        cuisine: "Greek".to_string(),
        borough: Some("Queens".to_string()),
    };

    let res = my_coll.insert_one(doc, None).await?;
    println!("Inserted a document with _id: {}", res.inserted_id);

    Ok(())
}

pub async fn insert_many(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let docs = vec![
        Restaurant {
            name: "While in Kathmandu".to_string(),
            cuisine: "Nepalese".to_string(),
            borough: None,
        },
        Restaurant {
            name: "Cafe Himalaya".to_string(),
            cuisine: "Nepalese".to_string(),
            borough: None,
        },
    ];

    let insert_many_result = my_coll.insert_many(docs, None).await?;
    println!("Inserted documents with _ids:");
    for (key, value) in &insert_many_result.inserted_ids {
        println!("key: {}, value: {}", key, value);
    }

    Ok(())
}
