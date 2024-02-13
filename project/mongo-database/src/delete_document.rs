use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

pub async fn delete_one(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! {"$and": [
        doc! {"name": "Tompkins Square Bagels"},
        doc! {"cuisine": "American"}
    ]};

    let result = my_coll.delete_one(filter, None).await?;
    println!("Deleted documents: {}", result.deleted_count);

    Ok(())
}

pub async fn delete_many(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! { "$and": [
           doc! { "name": "Cafe Un Deux Trois" },
           doc! { "cuisine": "French" }
       ]
    };

    let result = my_coll.delete_many(filter, None).await?;
    println!("Deleted documents: {}", result.deleted_count);

    Ok(())
}
