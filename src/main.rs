#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;

#[derive(Serialize)]
struct Website {
    title: String,
    mytext: String
}

#[get("/")]
fn index() -> Template {
    let context = Website {
        title: String::from("Ready to launch!"),
        mytext: String::from("Whoa!")
    };

    Template::render("index", &context)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}