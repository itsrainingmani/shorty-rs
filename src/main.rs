use dashmap::DashMap;
use rand::{thread_rng, Rng};
use rocket::{
    response::{
        status::{BadRequest, NotFound},
        Redirect,
    },
    State,
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/api/shorten?<url>")]
fn shorten(url: String, state: &State<DashMap<u32, String>>) -> Result<String, BadRequest<&str>> {
    if url.is_empty() {
        Err(BadRequest(Some("URL is empty")))
    } else {
        let key: u32 = thread_rng().gen();
        state.insert(key, url);
        Ok(key.to_string())
    }
}

#[get("/<key>")]
fn redirect(key: u32, state: &State<DashMap<u32, String>>) -> Result<Redirect, NotFound<&str>> {
    state
        .get(&key)
        .map(|url| Redirect::to(url.clone()))
        .ok_or(NotFound("Invalid or expired link!"))
}

#[launch]
fn rocket() -> _ {
    // dashmap is a fast, concurrent hashmap that implements Sync
    // we only need to access it from endpoints, so we let rocket manage it directly
    rocket::build()
        .manage(DashMap::<u32, String>::new())
        .mount("/", routes![shorten, redirect])
}
