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
    body: WebsiteBody
}

#[derive(Serialize)]
struct WebsiteBody {
    body_text_h1: String,
    body_text_h2: String
}

#[get("/")]
fn index() -> Template {
    let body_content = WebsiteBody {
        body_text_h1: String::from("Hello!"),
        body_text_h2: String::from("World!")
    };

    let context = Website {
        title: String::from("Ready to launch!"),
        body: body_content
    };

    Template::render("index", &context)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}