use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Fruit {
    _id: String,
    name: String,
    quantity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendors: Option<Vec<String>>,
}

pub async fn query(client: &Client) -> mongodb::error::Result<()> {
    let query = doc! {"vendors": doc! {"$elemMatch": {"$eq": "C"}}};

    let my_coll: Collection<Fruit> = client.database("sample_restaurants").collection("fruits");

    let mut cursor = my_coll.find(query, None).await?;
    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }

    Ok(())
}
