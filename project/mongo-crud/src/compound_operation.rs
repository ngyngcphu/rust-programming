use mongodb::{
    bson::{bson, doc, Document},
    options::{
        FindOneAndDeleteOptions, FindOneAndReplaceOptions, FindOneAndUpdateOptions, ReturnDocument,
    },
    Client, Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Student {
    name: String,
    age: u8,
    school: String,
}

pub async fn find_and_delete(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Student> = client.database("sample_restaurants").collection("students");

    let filter = doc! {"age": doc! {"$lte": 10}};
    let opts = FindOneAndDeleteOptions::builder()
        .comment(bson!("hello"))
        .build();

    let res = my_coll.find_one_and_delete(filter, opts).await?;
    println!("Deleted document:\n{:?}", res);

    Ok(())
}

pub async fn find_and_update(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Student> = client.database("sample_restaurants").collection("students");

    let filter = doc! { "school": "Aurora High School" };
    let update = doc! {
        "$set": doc! {"school": "Durango High School"},
        "$inc": doc! {"age": 1}
    };

    let opts = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let res = my_coll.find_one_and_update(filter, update, opts).await?;
    println!("Updated document:\n{:?}", res);

    Ok(())
}

pub async fn find_and_replace(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> =
        client.database("sample_restaurants").collection("students");

    let filter = doc! { "name": doc! { "$regex": "Toby" } };
    let replacement = doc! {
        "name": "Samara Khan",
        "age": 11,
        "school": "Rolling Hills Middle School"
    };
    let opts = FindOneAndReplaceOptions::builder()
        .return_document(Some(ReturnDocument::After))
        .projection(doc! { "name": 1, "school": 1, "_id": 0 })
        .build();

    let res = my_coll
        .find_one_and_replace(filter, replacement, opts)
        .await?;
    println!("Document after replacement:\n{:?}", res);

    Ok(())
}
