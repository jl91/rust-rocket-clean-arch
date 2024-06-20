#[macro_use]
extern crate rocket;

mod application;
mod domain;
mod infrastructure;

use std::any::TypeId;
use std::sync::Arc;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use application::entrypoints::rest::users_requests_handlers::*;
use crate::application::dependency_injection::Container;
use crate::application::repositories_impls::UserDomainRepositoryImpl;
use crate::application::state::AppState;
use crate::domain::shared::repositories::UserDomainRepository;
use crate::domain::usecases::list_users_usecase::ListUsersUsecase;
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

    // Database Dependencies
    di.register(establish_connection());

    di.register(
        UserDatabaseRepository::new(
            di.resolve::<Pool<ConnectionManager<PgConnection>>>().unwrap()
        )
    );

    // Domain Repository
    di.register_boxed(
        Box::new(
            UserDomainRepositoryImpl::new(
                di.resolve::<UserDatabaseRepository>().unwrap()
            )
        )
    );

    //Usercases
    // di.register(
    //     ListUsersUsecase::new(
    //         di.resolve_boxed::<dyn UserDomainRepository>().unwrap(),
    //     )
    // );

    di
}

