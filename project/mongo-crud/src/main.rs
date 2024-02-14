mod retrieve_data;
mod search_text;
mod specify_query;

use mongodb::Client;

#[tokio::main]
async fn main() {
    let uri = "mongodb://admin:admin123@14.225.192.183:27017";
    let client = Client::with_uri_str(uri).await.unwrap();

    println!("********* Query literal values *********");
    specify_query::query(&client).await.unwrap();

    println!("********* Search for a term *********");
    search_text::search(&client).await.unwrap();
}
