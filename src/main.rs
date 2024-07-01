#[macro_use]
extern crate rocket;

mod application;
mod domain;
mod infrastructure;

use shaku::module;
use application::entrypoints::rest::users_requests_handlers::*;
use crate::application::repositories_impls::UserDomainRepositoryImpl;
use crate::domain::shared::repositories::UserDomainRepository;
use crate::infrastructure::database::repositories::UserDatabaseRepository;

#[launch]
fn rocket() -> _ {
    let diModule = DiModule::builder().build();
    rocket::build()
        .manage(Box::new(diModule))
        .mount("/", routes![
        new_user,
        get_all,
        get_one,
        update_user,
        delete_user
    ])
}

module! {
    pub DiModule {
        components = [
            UserDomainRepositoryImpl,
            UserDatabaseRepository
        ],
        providers = [

        ]
    }
}


