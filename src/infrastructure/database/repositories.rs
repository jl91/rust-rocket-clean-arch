use std::error::Error;
use std::sync::Arc;
use chrono::{Utc};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use r2d2::PooledConnection;
use uuid::Uuid;
use crate::infrastructure::database::connection::ConnectionFactory;
use crate::infrastructure::database::entities::UserDatabaseEntity;
use crate::infrastructure::database::schemas::users::dsl::{users};
use crate::infrastructure::database::schemas::users::{created_at, deleted_at, external_id, password, updated_at, username};

pub trait DatabaseRepository<T, K> {
    fn create(&self, entity: T) -> QueryResult<T>;

    fn find_all(&self, size: Option<u64>, page: Option<u64>) -> QueryResult<Vec<T>>;

    fn find_one_by_id(&self, id: K) -> QueryResult<T>;

    fn update(&self, entity: T) -> QueryResult<T>;

    fn soft_delete(&self, id: Uuid) -> QueryResult<usize>;

    // fn delete(&self, id: K) -> QueryResult<usize>;
}

pub struct UserDatabaseRepository {
    connection_factory: Arc<dyn ConnectionFactory>,
}

impl UserDatabaseRepository {
    pub fn new(connection_factory: Arc<dyn ConnectionFactory>) -> Self {
        Self {
            connection_factory
        }
    }
}

impl DatabaseRepository<UserDatabaseEntity, Uuid> for UserDatabaseRepository {
    fn create(&self, new_user: UserDatabaseEntity) -> QueryResult<UserDatabaseEntity> {
            diesel::insert_into(users)
                .values((
                    username.eq(new_user.username),
                    password.eq(new_user.password),
                    created_at.eq(Utc::now().naive_utc()),
                ))
                .get_result::<UserDatabaseEntity>(&mut self.connection_factory.connect().get().unwrap())
    }

    fn find_all(&self, size: Option<u64>, page: Option<u64>) -> QueryResult<Vec<UserDatabaseEntity>>{
        let limit = size.unwrap_or(10);
        let offset = page.unwrap_or(0) * limit;

         users
            .limit(limit as i64)
            .offset(offset as i64)
            .filter(deleted_at.is_null())
            .load::<UserDatabaseEntity>(&mut self.connection_factory.connect().get().unwrap())
    }

    fn find_one_by_id(&self, user_id: Uuid) -> QueryResult<UserDatabaseEntity> {
        users.filter(external_id.eq(user_id))
            .first(&mut self.connection_factory.connect().get().unwrap())
    }

    fn update(&self, updated_user: UserDatabaseEntity) -> QueryResult<UserDatabaseEntity> {
        diesel::update(users.filter(external_id.eq(updated_user.external_id)))
            .set((
                username.eq(updated_user.username),
                password.eq(updated_user.password),
                updated_at.eq(Utc::now().naive_utc())
            ))
            .get_result(&mut self.connection_factory.connect().get().unwrap())
    }

    fn soft_delete(&self, user_id: Uuid) -> QueryResult<usize> {
      diesel::update(users.filter(external_id.eq(user_id)))
            .set(
                (deleted_at.eq(Utc::now().naive_utc()))
            )
            .execute(&mut self.connection_factory.connect().get().unwrap())
    }
}