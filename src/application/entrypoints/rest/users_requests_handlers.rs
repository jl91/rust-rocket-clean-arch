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
use rocket::response::status::Custom;
use crate::infrastructure::repositories_impls::UserDomainRepositoryImpl;
use crate::{DiContainer};
use crate::application::entrypoints::rest::{DefaultResponse};
use crate::domain::entities::{GenericQueryDomainEntity, UpdateUserDomainEntity, UserDomainEntity};
use crate::domain::shared::UsecaseSpecification;
use crate::domain::usecases::list_users_usecase::ListUsersUsecase;
use crate::infrastructure::database::repositories::UserDatabaseRepository;
use rocket::serde::{Deserialize, json::Json};
use crate::application::mappers::{new_user_from_dto_to_domain, update_user_from_dto_to_domain};


#[derive(Deserialize)]
pub struct NewUserRequestDTO {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequestDTO {
    pub username: String,
    pub password: String,
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn new_user(
    new_user: Json<NewUserRequestDTO>,
    state: &State<DiContainer>,
) -> Result<Custom<RawJson<String>>, Custom<String>> {

    let logger = state.get_custom_default_logger();

    logger.info(
        "users_requests_handler".to_string(),
        "new_user".to_string(),
        line!(),
        "iniciando criação de usuário".to_string()
    );

    let new_user_domain_entity = new_user_from_dto_to_domain(new_user.0);

    let data = state.create_user_usecase_instance()
        .execute(new_user_domain_entity);

    match data {
        Ok(users) => {

            let json_users = serde_json::to_string(&DefaultResponse {
                data: users
            });

            match json_users {
                Ok(data) => {

                    logger.info(
                        "users_requests_handler".to_string(),
                        "new_user".to_string(),
                        line!(),
                        "sucesso ao criar usuarios".to_string()
                    );

                    Ok(
                        Custom(
                            Status::Ok,
                            RawJson(data)
                        )
                    )
                }
                Err(err) =>{

                    logger.error(
                        "users_requests_handler".to_string(),
                        "new_user".to_string(),
                        line!(),
                        format!("erro ao criar usuario error: {:?}", err)
                    );

                    Err(
                        Custom(
                            Status::InternalServerError,
                            Status::InternalServerError.to_string()
                        )
                    )

                }
            }
        }
        Err(err) => {

            logger.error(
                "users_requests_handler".to_string(),
                "new_user".to_string(),
                line!(),
                format!("erro ao criar usuario error: {:?}", err)
            );

            Err(
                Custom(
                    Status::InternalServerError,
                    Status::InternalServerError.to_string()
                )
            )

        }
    }
}

#[get("/users?<page>&<size>")]
pub fn get_all(
    page: Option<u64>,
    size: Option<u64>,
    state: &State<DiContainer>,
) -> Result<Custom<RawJson<String>>, Custom<String>> {

    let logger = state.get_custom_default_logger();

    logger.info(
        "users_requests_handler".to_string(),
        "get_all".to_string(),
        line!(),
        "iniciando criação de usuário".to_string()
    );

    let data = state.list_users_usecase_instance()
        .execute(GenericQueryDomainEntity { page, size });

    match data {
        Ok(users) => {

            let json_users = serde_json::to_string(&DefaultResponse {
                data: users
            });

            match json_users {
                Ok(data) => {

                    logger.info(
                        "users_requests_handler".to_string(),
                        "get_all".to_string(),
                        line!(),
                        "sucesso ao listar usuarios".to_string()
                    );

                    Ok(
                        Custom(
                            Status::Ok,
                            RawJson(data)
                        )
                    )
                }
                Err(err) =>{

                    logger.error(
                        "users_requests_handler".to_string(),
                        "get_all".to_string(),
                        line!(),
                        format!("erro ao listar usuarios error: {:?}", err)
                    );

                    Err(
                        Custom(
                            Status::InternalServerError,
                            Status::InternalServerError.to_string()
                        )
                    )

                }
            }
        }
        Err(err) => {

            logger.error(
                "users_requests_handler".to_string(),
                "get_all".to_string(),
                line!(),
                format!("erro ao listar usuarios error: {:?}", err)
            );

            Err(
                Custom(
                    Status::InternalServerError,
                    Status::InternalServerError.to_string()
                )
            )

        }
    }
}


#[get("/users/<id>")]
pub fn get_one(
    id: String,
    state: &State<DiContainer>,
) -> Result<Custom<RawJson<String>>, Custom<String>>  {

    let logger = state.get_custom_default_logger();

    logger.info(
        "users_requests_handler".to_string(),
        "get_one".to_string(),
        line!(),
        format!("iniciando listagem de usuário id {:?}", id)
    );

    match Uuid::parse_str(&id) {
        Ok(id) => {

            let data = state.one_user_usecase_instance()
                .execute(
                    id
                );

            match data {
                Ok(users) => {

                    let json_users = serde_json::to_string(&DefaultResponse {
                        data: users
                    });

                    match json_users {
                        Ok(data) => {

                            logger.info(
                                "users_requests_handler".to_string(),
                                "get_one".to_string(),
                                line!(),
                                format!("sucesso ao listar usuario id: {:?}", id)
                            );

                            Ok(Custom(Status::Ok, RawJson(data)))

                        },
                        Err(err) =>{

                            logger.error(
                                "users_requests_handler".to_string(),
                                "get_one".to_string(),
                                line!(),
                                format!("erro ao listar usuarios id :{:?} error: {:?}", id, err)
                            );

                            Err(Custom(Status::InternalServerError, "Error".parse().unwrap()))
                        }
                    }

                }
                Err(err) => {

                    logger.error(
                        "users_requests_handler".to_string(),
                        "get_all".to_string(),
                        line!(),
                        format!("erro ao listar usuarios error: {:?}", err)
                    );

                    Err(
                        Custom(
                            Status::InternalServerError,
                            format!("Failed to list users: {:?}", err),
                        )
                    )
                },
            }

        }
        Err(err) => {

            logger.error(
                "users_requests_handler".to_string(),
                "get_one".to_string(),
                line!(),
                format!("erro ao listar usuarios id :{:?} error: {:?}", id, err)
            );

            Err(
                Custom(
                    Status::InternalServerError,
                    format!("Failed to list users: {:?}", err),
                )
            )
        }
    }
}

#[put("/users/<id>", format = "json", data = "<update_user>")]
pub fn update_user(
    id: String,
    update_user: Json<UpdateUserRequestDTO>,
    state: &State<DiContainer>,
) -> Result<Custom<RawJson<String>>, Custom<String>> {

    let logger = state.get_custom_default_logger();

    logger.info(
        "users_requests_handler".to_string(),
        "update_user".to_string(),
        line!(),
        format!("iniciando update de usuário id {:?}", id)
    );

    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            let data = update_user_from_dto_to_domain(uuid, update_user.0);

            let data = state.update_user_usecase_instance()
                .execute(
                    data
                );

            match data {
                Ok(users) => {

                    let json_users = serde_json::to_string(&DefaultResponse {
                        data: users
                    });

                    match json_users {
                        Ok(data) => {

                            logger.info(
                                "users_requests_handler".to_string(),
                                "update_user".to_string(),
                                line!(),
                                "sucesso ao atualizar usuário".to_string()
                            );

                            Ok(
                                Custom(
                                    Status::Ok,
                                    RawJson(data)
                                )
                            )

                        },
                        Err(err) =>{

                            logger.error(
                                "users_requests_handler".to_string(),
                                "update_user".to_string(),
                                line!(),
                                format!("erro ao listar usuarios error: {:?}", err)
                            );

                            Err(Custom(Status::InternalServerError, "Error".parse().unwrap()))
                        }
                    }
                }
                Err(err) => {

                    logger.error(
                        "users_requests_handler".to_string(),
                        "get_all".to_string(),
                        line!(),
                        format!("erro ao listar usuarios error: {:?}", err)
                    );

                    Err(
                        Custom(
                            Status::NotFound,
                            Status::NotFound.to_string(),
                        )
                    )
                },
            }
        }
        Err(err) => {

            logger.error(
                "users_requests_handler".to_string(),
                "update_user".to_string(),
                line!(),
                format!("erro ao atualizar usuario id :{:?} error: {:?}", id, err)
            );

            Err(
                Custom(
                    Status::InternalServerError,
                    Status::InternalServerError.to_string(),
                )
            )
        }
    }
}

#[delete("/users/<id>")]
pub fn delete_user(
    id: String,
    state: &State<DiContainer>,
) -> Result<Custom<RawJson<String>>, Custom<String>> {

    let logger = state.get_custom_default_logger();

    logger.info(
        "users_requests_handler".to_string(),
        "delete_user".to_string(),
        line!(),
        format!("iniciando remoção de usuário id {:?}", id)
    );

    match Uuid::parse_str(&id) {
        Ok(id) => {

            let data = state.delete_user_usecase_instance()
                .execute(
                    id
                );

            match data {
                Ok(users) => {

                    let json_users = serde_json::to_string(&DefaultResponse {
                        data: users
                    });

                    match json_users {
                        Ok(data) => {

                            logger.info(
                                "users_requests_handler".to_string(),
                                "delete_user".to_string(),
                                line!(),
                                format!("sucesso ao apagar usuario id: {:?}", id)
                            );

                            Ok(
                                Custom(
                                    Status::Ok,
                                    RawJson(data)
                                )
                            )

                        },
                        Err(err) =>{

                            logger.error(
                                "users_requests_handler".to_string(),
                                "delete_user".to_string(),
                                line!(),
                                format!("erro ao apagar usuario id :{:?} error: {:?}", id, err)
                            );

                            Err(
                                Custom(
                                    Status::InternalServerError,
                                    Status::InternalServerError.to_string()
                                )
                            )
                        }
                    }

                }
                Err(err) => {

                    logger.error(
                        "users_requests_handler".to_string(),
                        "delete_user".to_string(),
                        line!(),
                        format!("erro ao listar usuarios error: {:?}", err)
                    );

                    Err(
                        Custom(
                            Status::InternalServerError,
                            format!("Failed to list users: {:?}", err),
                        )
                    )
                },
            }

        }
        Err(err) => {

            logger.error(
                "users_requests_handler".to_string(),
                "delete_user".to_string(),
                line!(),
                format!("erro ao listar usuarios id :{:?} error: {:?}", id, err)
            );

            Err(
                Custom(
                    Status::InternalServerError,
                    Status::InternalServerError.to_string(),
                )
            )
        }
    }
}
