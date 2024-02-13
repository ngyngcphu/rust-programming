use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

pub async fn count_documents(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let ct = my_coll.estimated_document_count(None).await?;
    println!("Number of documents: {}", ct);

    let ct = my_coll
        .count_documents(doc! { "name": doc! { "$regex": "Calliope" } }, None)
        .await?;
    println!("Number of matching documents: {}", ct);

    Ok(())
}

pub async fn list_distinct_field_values(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! {"cuisine": "Greek"};
    let boroughs = my_coll.distinct("borough", filter, None).await?;

    println!("List of field values for 'borough':");
    for b in boroughs {
        println!("{:?}", b);
    }
    Ok(())
}
