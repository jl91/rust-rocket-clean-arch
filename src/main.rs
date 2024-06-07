#[macro_use]
extern crate rocket;

mod application;
mod domain;
mod infrastructure;

use application::entrypoints::rest::users_requests_handlers::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        new_user,
        get_all,
        get_one,
        update_user,
        delete_user
    ])
}