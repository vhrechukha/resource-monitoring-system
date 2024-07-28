use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, from_document, to_document, Document},
    options::ClientOptions,
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    description: String,
}

async fn get_items(client: web::Data<Client>) -> impl Responder {
    let collection = client.database("test").collection::<Document>("items");
    let mut cursor = collection.find(doc! {}).await.unwrap();

    let mut items: Vec<Item> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                let item: Item = from_document(doc).unwrap();
                items.push(item);
            }
            Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }

    HttpResponse::Ok().json(items)
}

async fn add_item(client: web::Data<Client>, item: web::Json<Item>) -> impl Responder {
    let collection = client.database("test").collection("items");
    let new_item = Item {
        name: item.name.clone(),
        description: item.description.clone(),
    };

    let doc = to_document(&new_item).unwrap();
    collection.insert_one(doc).await.unwrap();

    HttpResponse::Ok().json(new_item)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(add_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
