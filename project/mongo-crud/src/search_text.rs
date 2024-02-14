use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection, IndexModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Dish {
    name: String,
    description: String,
}

pub async fn search(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Dish> = client.database("sample_restaurants").collection("dishes");
    let index = IndexModel::builder()
        .keys(doc! {"description": "text"})
        .build();

    my_coll.create_index(index, None).await?;

    let filter = doc! { "$text": { "$search": "vegan -tofu" } };
    let mut cursor = my_coll.find(filter, None).await?;
    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }

    Ok(())
}
