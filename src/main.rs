use dashmap::DashMap;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    // dashmap is a fast, concurrent hashmap that implements Sync
    // we only need to access it from endpoints, so we let rocket manage it directly
    rocket::build()
        .manage(DashMap::<u32, String>::new())
        .mount("/", routes![index])
}
