
mod syncer;

mod datasource;

#[macro_use] extern crate rocket;
//use std::path::{ Path,PathBuf};
//use rocket::Error;
//use rocket::fs::NamedFile;
use rocket::fs::FileServer;
//use rocket::tokio::time::{sleep,Duration };
// use std::fs;
// use serde::{Serialize,Deserialize};
use crate::datasource::contact;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/sync")]
async fn sync(){
    if let Err(err) = syncer::sync_contacts().await{
        println!("sync failed: {}", err);

    }
    println!("OK")

}

// #[get("/<file..>")]
// async fn files(file: PathBuf) -> Option<NamedFile> {
//    NamedFile::open(Path::new("static/").join(file)).await.ok()
// }

#[launch]
fn rocket() -> _ {
    rocket::build().
        mount("/", routes![index,sync]).
        mount("/public", FileServer::from("www/static"))
}

 // fn main() {
 //     let json_string = fs::read_to_string("static/contact_list.json").unwrap();
 //     //let json_string = fs::read_to_string("static/contact_frank.json").unwrap();
 //     let person_list: Vec<contact::Person> = serde_json::from_str(&json_string).unwrap();
 //     println!("last person: \n\n {:?}",person_list[person_list.len()-1]);
 //     println!("Hello, world!");
 // }
