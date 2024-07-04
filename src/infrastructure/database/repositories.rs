use std::sync::Arc;
use std::vec::IntoIter;
use chrono::{Utc};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;
use crate::infrastructure::database::connection::ConnectionFactory;
use crate::infrastructure::database::entities::UserDatabaseEntity;
use crate::infrastructure::database::schemas::users::dsl::{users};
use crate::infrastructure::database::schemas::users::{created_at, external_id, password, updated_at, username};

pub trait DatabaseRepository<T, K> {
    fn create(&self, entity: T) -> QueryResult<T>;

    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> IntoIter<T>;

    fn find_one_by_id(&self, id: K) -> QueryResult<T>;

    fn update(&self, entity: T) -> QueryResult<T>;
    //
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

    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> IntoIter<UserDatabaseEntity> {
        let limit = size.unwrap_or(10);
        let offset = page.unwrap_or(0) * limit;
        let results = users
            .limit(limit)
            .offset(offset)
            .load::<UserDatabaseEntity>(&mut self.connection_factory.connect().get().unwrap())
            .expect("Error loading users");

        results.into_iter()
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
                updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result(&mut self.connection_factory.connect().get().unwrap())
    }

    // fn delete(&self, user_id: Uuid) -> QueryResult<usize> {
    //     diesel::delete(users.filter(external_id.eq(user_id)))
    //         .execute(self.conn)
    // }
}