use mongodb::{bson::doc, Client};

pub async fn connect(client: &Client) -> mongodb::error::Result<()> {
    client
        .database("admin")
        .run_command(doc! { "ping": 1 }, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(())
}
