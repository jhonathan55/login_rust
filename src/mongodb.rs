
use futures::StreamExt;
use mongodb::{
    bson::doc,
    error::Error,
    options::{ClientOptions, IndexOptions},
    Client, Collection, IndexModel,
};

use crate::structs::structs::User;
pub async fn mongodb_init() -> Result<Collection<User>, Error> {
    /// Esta función se encarga de crear la colección y el índice en la base de datos.
    /// Si la colección ya existe, no se crea.
    /// Si el índice ya existe, no se crea.
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"email": 1})
        .options(options)
        .build();
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("login_rust");
    let collections = db.list_collection_names(doc! {"name": "users"}).await?;
    if collections.is_empty() {
        let _ = match db.create_collection("users", None).await {
            Ok(_) => println!("La colección se creó correctamente."),
            Err(e) => {
                println!("Error al crear la colección: {:?}", e.to_string());
                return Err(e);
            }
        };
    }
    let collection = db.collection::<User>("users");
    let index_names = collection.list_indexes(None).await?;
    let mut index_exists = false;
    index_names
        .for_each(|index| {
            let index = match index {
                Ok(index) => index,
                Err(e) => {
                    println!("Error al obtener el índice: {:?}", e.to_string());
                    return futures::future::ready(());
                }
            };
            let index_options = match index.options.as_ref() {
                Some(indexoptions) => indexoptions,
                None => {
                    println!("No se encontró el índice");
                    return futures::future::ready(());
                }
            };
            let name = match index_options.name.clone() {
                Some(name) => name,
                None => {
                    println!("No se encontró el nombre del índice");
                    return futures::future::ready(());
                }
            };
            if name == "email_1" {
                index_exists = true;
            }

            futures::future::ready(())
        })
        .await;

    if !index_exists {
        match collection.create_index(model, None).await {
            Ok(index) => {
                println!("El índice se creó correctamente: {:?}", index);
            }
            Err(e) => {
                println!("Error al crear el índice: {:?}", e.to_string());
                return Err(e);
            }
        };
    }

    Ok(collection)
}

// pub async fn insert_one<T>(
//     collection: &Collection<T>,
//     document: T,
// ) -> Result<mongodb::results::InsertOneResult, Error>
// where
//     T: serde::Serialize,
// {
//     collection.insert_one(document, None).await
// }
