use mongodb::Client;

mod connection;
mod find_document;

#[tokio::main]
async fn main() {
    let uri = "mongodb://admin:admin123@14.225.192.183:27017";
    let client = Client::with_uri_str(uri).await.unwrap();

    connection::connect(&client).await.unwrap();

    println!("**********Find a document**********");
    find_document::find_one(&client).await.unwrap();

    println!("*********Find multiple documents*********");
    find_document::find_many(&client).await.unwrap();
}
