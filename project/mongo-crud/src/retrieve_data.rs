use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{FindOneOptions, FindOptions},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Inventory {
    item: String,
    category: String,
    unit_price: f64,
}

pub async fn find_many(client: &Client) -> mongodb::error::Result<()> {
    let opts = FindOptions::builder().sort(doc! {"unit_price": -1}).build();

    let my_coll: Collection<Inventory> = client
        .database("sample_restaurants")
        .collection("inventory");

    let mut cursor = my_coll
        .find(
            doc! { "$and": vec!
            [
                doc! { "unit_price": doc! { "$lt": 12.00 } },
                doc! { "category": doc! { "$ne": "kitchen" } }
            ] },
            opts,
        )
        .await
        .unwrap();

    while let Some(result) = cursor.try_next().await.unwrap() {
        println!("{:?}", result);
    }

    Ok(())
}

pub async fn find_one(client: &Client) -> mongodb::error::Result<()> {
    let opts = FindOneOptions::builder().skip(2).build();

    let my_coll: Collection<Inventory> = client
        .database("sample_restaurants")
        .collection("inventory");

    let result = my_coll
        .find_one(
            doc! { "unit_price":
            doc! { "$lte": 20.00 } },
            opts,
        )
        .await?;

    println!("{:#?}", result);

    Ok(())
}

pub async fn aggregate(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Inventory> = client
        .database("sample_restaurants")
        .collection("inventory");

    let pipeline = vec![
        doc! {"$group": doc! {"_id": doc! {"category": "$category"}, "avg_price": doc! {"$avg": "$unit_price"}}},
        doc! {"$sort": {"_id.avg_price": 1}},
    ];

    let mut cursor = my_coll.aggregate(pipeline, None).await?;
    while let Some(result) = cursor.try_next().await? {
        println!("{:?}", result);
    }
    Ok(())
}
