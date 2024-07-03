use std::fmt::format;
use std::os::macos::raw::stat;
use std::ptr::read;
use std::sync::Arc;
use uuid::Uuid;
use rocket::{get, post, put, delete, http::Status, Response, State};
use rocket::http::ContentType;
use rocket::response::content;
use rocket::response::content::RawJson;
use crate::application::repositories_impls::UserDomainRepositoryImpl;
use crate::{DiContainer};
use crate::application::entrypoints::rest::UserRequestDTO;
// use crate::application::mappers::from_user_dtoto_domain_entity;
use crate::domain::entities::{GenericQueryDomainEntity, UserDomainEntity};
use crate::domain::shared::UsecaseSpecification;
use crate::domain::usecases::list_users_usecase::ListUsersUsecase;
use crate::infrastructure::database::repositories::UserDatabaseRepository;
use rocket::serde::{Deserialize, json::Json};

// #[post("/users",format="json", data = "<new_user>")]
// pub fn new_user(
//     new_user: Json<UserRequestDTO>,
//     state: &State<DiContainer>
// ) -> RawJson<String> {
//
//     let user_domain_entity = from_user_dtoto_domain_entity(new_user.0);
//
//     let data =  state.create_user_usecase_instance()
//         .execute(user_domain_entity);
//
//     let data_json = serde_json::to_string(&data.unwrap())
//         .expect("Failed to serialize users to JSON");
//
//     RawJson(data_json)
// }

#[get("/users?<page>&<size>")]
pub fn get_all(
    page: Option<i64>,
    size: Option<i64>,
    state: &State<DiContainer>
) -> content::RawJson<String> {
    let data = state.list_users_usecase_instance()
        .execute(
        GenericQueryDomainEntity {
            page,
            size,
        }
    );

    let data_json = serde_json::to_string(&data.unwrap())
        .expect("Failed to serialize users to JSON");

    content::RawJson(data_json)
}

#[get("/users/<id>")]
pub fn get_one(
    id: String,
    state: &State<DiContainer>
) -> content::RawJson<String> {
    let data = state.one_user_usecase_instance()
        .execute(
            Uuid::parse_str(&id).unwrap()
        );

    let data_json = serde_json::to_string(&data.unwrap())
        .expect("Failed to serialize users to JSON");

    content::RawJson(data_json)
}

#[put("/users/<id>")]
pub fn update_user(id: String) -> content::RawJson<String> {
    content::RawJson(format!("{{\"message\": \"test update one user by id {}\"}}", id.to_string()))
}

#[delete("/users/<id>")]
pub fn delete_user(id: String) -> content::RawJson<String> {
    content::RawJson(format!("{{\"message\": \"test delete one user by id {}\"}}", id.to_string()))
}
