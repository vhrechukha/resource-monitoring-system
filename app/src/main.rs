use std::env;
use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, from_document, to_document, Document},
    options::ClientOptions,
    Client,
};
use serde::{Deserialize, Serialize};
use telegraf::{Client as TelegrafClient, IntoFieldData, Point};

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

async fn add_item(
    client: web::Data<Client>,
    item: web::Json<Item>,
    telegraf_client: web::Data<Arc<Mutex<TelegrafClient>>>,
) -> impl Responder {
    let collection = client.database("test").collection("items");
    let new_item = Item {
        name: item.name.clone(),
        description: item.description.clone(),
    };

    let doc = to_document(&new_item).unwrap();
    collection.insert_one(doc).await.unwrap();

    let p = Point::new(
        "item_added".to_owned(),
        vec![("name".to_string(), new_item.name.clone())],
        vec![
            (
                "description".to_string(),
                Box::new(new_item.description.clone()) as Box<dyn IntoFieldData>,
            ),
            ("value".to_string(), Box::new(1.0) as Box<dyn IntoFieldData>),
        ],
        None,
    );

    let mut telegraf_client = telegraf_client.lock().unwrap();
    telegraf_client.write_point(&p).unwrap();

    HttpResponse::Ok().json(new_item)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
    let telegraf_url = env::var("TELEGRAF_URL").expect("TELEGRAF_URL must be set");

    println!("Mongo URL: {}", mongo_url);
    println!("Telegraf URL: {}", telegraf_url);

    let client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let telegraf_client = Arc::new(Mutex::new(TelegrafClient::new(&telegraf_url).unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(telegraf_client.clone()))
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(add_item))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
