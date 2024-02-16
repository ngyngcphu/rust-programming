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

    println!("********* Find and Delete a Document *********");
    compound_operation::find_and_delete(&client).await.unwrap();

    println!("********* Find and Update a Document *********");
    compound_operation::find_and_update(&client).await.unwrap();

    println!("********* Find and Replace a Document *********");
    compound_operation::find_and_replace(&client).await.unwrap();
}
