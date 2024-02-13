use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    borough: String,
    cuisine: String,
    name: String,
}

pub async fn replace_one(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! {"name": "Cafe Himalaya"};
    let replacement = Restaurant {
        borough: "Brooklyn".to_string(),
        cuisine: "Café/Coffee/Tea".to_string(),
        name: "Harvest Moon Café".to_string(),
    };

    let res = my_coll.replace_one(filter, replacement, None).await?;
    println!("Replaced documents: {}", res.modified_count);

    Ok(())
}
