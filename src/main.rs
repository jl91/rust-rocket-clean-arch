#[macro_use]
extern crate rocket;

mod application;
mod domain;
mod infrastructure;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use application::entrypoints::rest::users_requests_handlers::*;
use crate::application::dependency_injection::Container;
use crate::application::repositories_impls::UserDomainRepositoryImpl;
use crate::application::state::AppState;
use crate::infrastructure::database::connection::establish_connection;
use crate::infrastructure::database::repositories::UserDatabaseRepository;

#[launch]
fn rocket() -> _ {
    let app_state = AppState::new();
    configure_di(&app_state.di_container);
    rocket::build()
        .manage(AppState::new())
        .mount("/", routes![
        new_user,
        get_all,
        get_one,
        update_user,
        delete_user
    ])
}

fn configure_di(di: &Container) -> &Container {
    di.register(establish_connection());


    // Database Dependencies
    di.register(
        UserDatabaseRepository::new(
            di.resolve::<Pool<ConnectionManager<PgConnection>>>().unwrap().clone()
        )
    );

    di.register(
        UserDomainRepositoryImpl::new(
            di.resolve::<UserDatabaseRepository>().unwrap()
        )
    );

    di
}

