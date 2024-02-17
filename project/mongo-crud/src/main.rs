mod aggregation;
mod compound_operation;
mod insert;
mod retrieve_data;
mod search_text;
mod specify_query;

use mongodb::Client;

#[tokio::main]
async fn main() {
    let uri = "mongodb://admin:admin123@14.225.192.183:27017";
    let client = Client::with_uri_str(uri).await.unwrap();

    println!("********* Group by Time Component *********");
    aggregation::aggregate_time(&client).await.unwrap();
}
