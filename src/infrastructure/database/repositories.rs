use std::sync::Arc;
use std::vec::IntoIter;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use shaku::{Component, Interface};
use uuid::Uuid;
use crate::infrastructure::database::connection::ConnectionFactory;
use crate::infrastructure::database::entities::UserDatabaseEntity;
use crate::infrastructure::database::schemas::users::dsl::{users};

pub trait DatabaseRepository<T, K> : Interface {

    //fn create(&self, entity: &T) -> QueryResult<T>;

    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> IntoIter<T>;

    // fn find_one_by_id(&self, id: K) -> QueryResult<T>;
    //
    // // fn update(&self, entity: &T) -> QueryResult<T>;
    //
    // fn delete(&self, id: K) -> QueryResult<usize>;
}

#[derive(Component)]
#[shaku(interface = DatabaseRepository<UserDatabaseEntity, Uuid>)]
pub struct UserDatabaseRepository {
    connection_factory: Arc<dyn ConnectionFactory>,
}

impl DatabaseRepository<UserDatabaseEntity, Uuid> for UserDatabaseRepository {

    // fn create(&self, new_user: &User) -> QueryResult<User> {
    //     diesel::insert_into(users)
    //         .values(new_user)
    //         .get_result(self.conn)
    // }

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

    // fn find_one_by_id(&self, user_id: Uuid) -> QueryResult<User> {
    //     users.filter(external_id.eq(user_id))
    //         .first(self.conn)
    // }

    // fn update(&self, updated_user: &User) -> QueryResult<User> {
    //     diesel::update(users.filter(updated_user.external_id))
    //         .set()
    //         .get_result(self.conn)
    // }

    // fn delete(&self, user_id: Uuid) -> QueryResult<usize> {
    //     diesel::delete(users.filter(external_id.eq(user_id)))
    //         .execute(self.conn)
    // }
}