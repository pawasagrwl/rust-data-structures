#[macro_use] extern crate rocket; 

use rocket::{tokio::sync::broadcast::{channel, Sender}, serde::{Serialize, Deserialize}};
use rocket::form::Form;

#[get("/world")]
fn world() -> &'static str {
    "Hello, World!"
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    let _res = queue.send(form.into_inner())
}

#[get("/event")]

# [launch]
fn rocket() ->  _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/hello", routes![world])
}
