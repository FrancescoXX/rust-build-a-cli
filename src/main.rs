#[macro_use]
extern crate rocket;

use rocket::response::content;

// Define a simple route for the root path "/"
#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket web server!"
}

// Define a route with a query parameter "?name="
#[get("/hello?<name>")]
fn hello_query(name: String) -> content::Json<String> {
    content::Json(format!("Hello, {}!", name))

}


// Define the main function to launch the Rocket web server
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}
