use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

pub async fn update_one(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! {"name": "Sea Stone Tavern"};
    let update = doc! {"$set": doc! {"price": "$$$"}};

    let res = my_coll.update_one(filter, update, None).await?;
    println!("Updated documents: {}", res.modified_count);

    Ok(())
}

pub async fn update_many(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! {"borough": "Queens"};
    let update = doc! {"$set": doc! {"near_me": true}};

    let res = my_coll.update_many(filter, update, None).await?;
    println!("Updated documents: {}", res.modified_count);

    Ok(())
}
