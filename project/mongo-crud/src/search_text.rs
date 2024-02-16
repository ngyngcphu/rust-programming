use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    Client, Collection, IndexModel,
};
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

    let filter = doc! { "$text": { "$search": "vegetarian" } };
    let sort = doc! {"score": { "$meta": "textScore" }};
    let projection = doc! {"_id": 0, "name": 1, "score": {"$meta": "textScore"}};
    let opts = FindOptions::builder()
        .sort(sort)
        .projection(projection)
        .build();

    let doc_coll: Collection<Document> = my_coll.clone_with_type();

    let mut cursor = doc_coll.find(filter, opts).await?;
    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }

    Ok(())
}
