#[macro_use]
extern crate rocket;

mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;
use log::LevelFilter;
use crate::application::entrypoints::rest::users_requests_handlers::{delete_user, get_all, get_one, new_user, update_user};
use infrastructure::repositories_impls::UserDomainRepositoryImpl;
use crate::domain::shared::repositories::UserDomainRepository;
use crate::domain::usecases::list_users_usecase::ListUsersUsecase;
use crate::domain::usecases::create_user_usecase::CreateUserUsecase;
use crate::domain::usecases::delete_user_usecase::DeleteUserUsecase;
use crate::domain::usecases::one_user_usecase::OneUsersUsecase;
use crate::domain::usecases::update_user_usecase::UpdateUserUsecase;
use crate::infrastructure::database::connection::{ConnectionFactory, ConnectionFactoryImpl};
use crate::infrastructure::database::repositories::{DatabaseRepository, UserDatabaseRepository};

#[launch]
fn rocket() -> _ {
    config_log();
    rocket::build()
        .manage(DiContainer::new())
        .mount("/", routes![
        new_user,
        get_all,
        get_one,
        update_user,
        delete_user
    ])
}

struct DiContainer;

impl DiContainer {

    fn new() -> Self {
        Self
    }

    //Logger
    fn get_custom_default_logger(&self) -> Arc<dyn domain::shared::repositories::Logger> {
        Arc::new(
            infrastructure::repositories_impls::DefaultLogger::new(
                json_log::get_default_logger()
            )
        )
    }

    // Database connection
    fn get_connection_factory(&self) -> Arc<dyn ConnectionFactory> {
        Arc::new(
            ConnectionFactoryImpl::new()
        )
    }

    // Database Repositories
    fn user_database_instance(&self) -> Arc<UserDatabaseRepository> {
        Arc::new(
            UserDatabaseRepository::new(
                self.get_connection_factory().clone()
            )
        )
    }

    // Domain Repositories
    fn user_domain_instance(&self) -> Arc<dyn UserDomainRepository> {
        Arc::new(
            UserDomainRepositoryImpl::new(
                self.user_database_instance()
            )
        )
    }

    // Usecases
    fn list_users_usecase_instance(&self) -> ListUsersUsecase {
        ListUsersUsecase::new(
            self.user_domain_instance(),
            self.get_custom_default_logger()
        )
    }

    fn create_user_usecase_instance(&self) -> CreateUserUsecase {
        CreateUserUsecase::new(
            self.user_domain_instance(),
            self.get_custom_default_logger()
        )
    }

    fn one_user_usecase_instance(&self) -> OneUsersUsecase {
        OneUsersUsecase::new(
            self.user_domain_instance(),
            self.get_custom_default_logger()
        )
    }

    fn update_user_usecase_instance(&self) -> UpdateUserUsecase {
        UpdateUserUsecase::new(
            self.user_domain_instance(),
            self.get_custom_default_logger()
        )
    }

    fn delete_user_usecase_instance(&self) -> DeleteUserUsecase {
        DeleteUserUsecase::new(
            self.user_domain_instance(),
            self.get_custom_default_logger()
        )
    }

}

fn config_log() {
    json_log::init_with_level(LevelFilter::Debug).unwrap();
}