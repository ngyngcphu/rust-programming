use mongodb::Client;

mod connection;
mod count_document;
mod delete_document;
mod find_document;
mod insert_document;
mod replace_document;
mod update_document;

#[tokio::main]
async fn main() {
    let uri = "mongodb://admin:admin123@localhost:27017";
    let client = Client::with_uri_str(uri).await.unwrap();

    connection::connect(&client).await.unwrap();

    println!("********** Find a document **********");
    find_document::find_one(&client).await.unwrap();

    println!("********* Find multiple documents *********");
    find_document::find_many(&client).await.unwrap();

    println!("********* Insert a document *********");
    insert_document::insert_one(&client).await.unwrap();

    println!("********* Insert multiple documents *********");
    insert_document::insert_many(&client).await.unwrap();

    println!("********* Update a document *********");
    update_document::update_one(&client).await.unwrap();

    println!("********* Update multiple documents *********");
    update_document::update_many(&client).await.unwrap();

    println!("********* Replace a document *********");
    replace_document::replace_one(&client).await.unwrap();

    println!("********* Delete a document *********");
    delete_document::delete_one(&client).await.unwrap();

    println!("********* Delete multiple documents *********");
    delete_document::delete_many(&client).await.unwrap();

    println!("********* Count documents *********");
    count_document::count_documents(&client).await.unwrap();

    println!("********* List distinct field values *********");
    count_document::list_distinct_field_values(&client)
        .await
        .unwrap();
}
