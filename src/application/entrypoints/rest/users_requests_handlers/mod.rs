use std::fmt::format;
use uuid::Uuid;
use rocket::{get, post, put, delete, http::Status, Response};
use rocket::http::ContentType;
use rocket::response::content;
use serde_json::{json, Value};
use crate::domain::entities::{QueryRequestDomainEntity, UserDomainEntity};
// use crate::domain::shared::UsecaseSpecification;
// use crate::domain::usecases::list_users_usecase::ListUsersUsecase;


#[post("/users")]
pub fn new_user() -> content::RawJson<&'static str> {
    content::RawJson("{\"message\": \"test post users\"}")
}

#[get("/users?<page>&<size>")]
pub fn get_all(
    page: Option<i64>,
    size: Option<i64>,
    // list_users_usecase: Inject<Dependencies>
) -> content::RawJson<String> {
    // let data = list_users_usecase.execute(
    //     QueryRequestDomainEntity{
    //         page,
    //         size
    //     }
    // );

    // let data_json = serde_json::to_string(&data)
    //     .expect("Failed to serialize users to JSON");
    //
    // content::RawJson(data_json)
    content::RawJson(format!("{{\"message\": \"test get one user by id {}\"}}", ""))

}

#[get("/users/<id>")]
pub fn get_one(id: String) -> content::RawJson<String> {
    content::RawJson(format!("{{\"message\": \"test get one user by id {}\"}}", id.to_string()))
}

#[put("/users/<id>")]
pub fn update_user(id: String) -> content::RawJson<String> {
    content::RawJson(format!("{{\"message\": \"test update one user by id {}\"}}", id.to_string()))
}

#[delete("/users/<id>")]
pub fn delete_user(id: String) -> content::RawJson<String> {
    content::RawJson(format!("{{\"message\": \"test delete one user by id {}\"}}", id.to_string()))
}
