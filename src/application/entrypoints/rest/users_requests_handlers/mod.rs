use std::error::Error;
use std::fmt::format;
use std::os::macos::raw::stat;
use std::ptr::read;
use std::sync::Arc;
use uuid::Uuid;
use rocket::{get, post, put, delete, http::Status, Response, State};
use rocket::http::ContentType;
use rocket::response::{content, status};
use rocket::response::content::RawJson;
use crate::infrastructure::repositories_impls::UserDomainRepositoryImpl;
use crate::{DiContainer};
use crate::application::entrypoints::rest::{NewUserRequestDTO, UpdateUserRequestDTO};
use crate::domain::entities::{GenericQueryDomainEntity, UpdateUserDomainEntity, UserDomainEntity};
use crate::domain::shared::UsecaseSpecification;
use crate::domain::usecases::list_users_usecase::ListUsersUsecase;
use crate::infrastructure::database::repositories::UserDatabaseRepository;
use rocket::serde::{Deserialize, json::Json};
use crate::application::mappers::{new_user_from_dto_to_domain, update_user_from_dto_to_domain};

#[post("/users", format = "json", data = "<new_user>")]
pub fn new_user(
    new_user: Json<NewUserRequestDTO>,
    state: &State<DiContainer>,
) -> RawJson<String> {
    let new_user_domain_entity = new_user_from_dto_to_domain(new_user.0);

    let data = state.create_user_usecase_instance()
        .execute(new_user_domain_entity)
        .expect("Failed to create user");

    let data_json = serde_json::to_string(&data)
        .expect("Failed to serialize users to JSON");

    RawJson(data_json)
}

#[get("/users?<page>&<size>")]
pub fn get_all(
    page: Option<u64>,
    size: Option<u64>,
    state: &State<DiContainer>,
) -> Result<content::RawJson<String>, status::Custom<String>> {
    let data = state.list_users_usecase_instance()
        .execute(GenericQueryDomainEntity { page, size });

    match data {
        Ok(users) => {
            let json_users = serde_json::to_string(&users)
                .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))?;
            Ok(RawJson(json_users))
        }
        Err(err) => Err(
            status::Custom(
                Status::InternalServerError,
                format!("Failed to list users: {}", err)
            )
        ),
    }
}

#[get("/users/<id>")]
pub fn get_one(
    id: String,
    state: &State<DiContainer>,
) -> Result<RawJson<String>, status::Custom<String>> {
    let data = state.one_user_usecase_instance()
        .execute(
            Uuid::parse_str(&id).unwrap()
        );

    match data {
        Ok(users) => {
            let json_users = serde_json::to_string(&users)
                .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))?;
            Ok(RawJson(json_users))
        }
        Err(err) => Err(
            status::Custom(
                Status::InternalServerError,
                format!("Failed to list user: {}", id)
            )
        ),
    }
}

#[put("/users/<id>", format = "json", data = "<update_user>")]
pub fn update_user(
    id: String,
    update_user: Json<UpdateUserRequestDTO>,
    state: &State<DiContainer>,
) -> Result<RawJson<String>, status::Custom<String>> {
    let data = update_user_from_dto_to_domain(id.clone(), update_user.0);

    let data = state.update_user_usecase_instance()
        .execute(
            data
        );

    match data {
        Ok(users) => {
            let json_users = serde_json::to_string(&users)
                .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))?;
            Ok(RawJson(json_users))
        }
        Err(err) => Err(
            status::Custom(
                Status::InternalServerError,
                format!("Failed to update user: {}", id.clone())
            )
        ),
    }
}

#[delete("/users/<id>")]
pub fn delete_user(
    id: String,
    state: &State<DiContainer>,
) -> Result<RawJson<String>, status::Custom<String>> {
    let data = state.delete_user_usecase_instance()
        .execute(
            Uuid::parse_str(&id).unwrap()
        );

    match data {
        Ok(users) => {
            let json_users = serde_json::to_string(&users)
                .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))?;
            Ok(RawJson(json_users))
        }
        Err(err) => Err(
            status::Custom(
                Status::InternalServerError,
                format!("Failed to delete user id: {}", id.clone())
            )
        ),
    }
}
