use futures::TryStreamExt;
use mongodb::{
    bson::{doc, serde_helpers, DateTime, Document},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct LastActive {
    #[serde(rename = "$date", serialize_with = "serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    date: DateTime,
}

#[derive(Debug, Deserialize, Serialize)]
struct BookProfile {
    name: String,
    age: u32,
    genre_interests: Vec<String>,
    last_active: LastActive,
}

pub async fn aggregate_age(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("book_profile");

    let age_pipeline = vec![
        doc! {"$unwind": doc! {"path": "$genre_interests"}},
        doc! {"$group": doc! {
            "_id": "$genre_interests",
            "avg_age": doc! {"$avg": "$age"},
            "min_age": doc! {"$min": "$age"},
            "max_age": doc! {"$max": "$age"},
        }},
    ];

    let mut result = my_coll.aggregate(age_pipeline, None).await?;
    while let Some(result) = result.try_next().await? {
        println!("* {:?}", result);
    }

    Ok(())
}

pub async fn aggregate_time(client: &Client) -> mongodb::error::Result<()> {
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("book_profile");

    let last_active_pipeline = vec![
        doc! { "$project": { "month_last_active": doc! { "$month": "$last_active" } } },
        doc! { "$group": doc! { "_id": doc! { "month_last_active": "$month_last_active" },
        "number": doc! { "$sum": 1 } } },
        doc! { "$sort": { "_id.month_last_active": 1 } },
    ];

    let mut result = my_coll.aggregate(last_active_pipeline, None).await?;
    while let Some(result) = result.try_next().await? {
        println!("* {:?}", result);
    }

    Ok(())
}
