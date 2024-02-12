use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Restaurant {
    name: String,
    cuisine: String,
}

pub async fn find_one(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let result = my_coll
        .find_one(doc! {"name": "Tompkins Square Bagels"}, None)
        .await?;

    println!("{:#?}", result.unwrap());

    Ok(())
}

pub async fn find_many(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let mut cursor = my_coll.find(doc! {"cuisine": "French"}, None).await?;
    while let Some(doc) = cursor.try_next().await? {
        println!("{:#?}", doc);
    }

    Ok(())
}
