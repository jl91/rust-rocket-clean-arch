use std::vec::IntoIter;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;
use crate::infrastructure::database::entities::User;
use crate::infrastructure::database::schemas::users::dsl::{users, external_id};

pub trait Repository<T, K> {
    fn new(conn: &mut PgConnection) -> Self;

    fn create(&self, entity: &T) -> QueryResult<T>;

    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> IntoIter<T>;

    fn find_one_by_id(&self, id: K) -> QueryResult<T>;

    // fn update(&self, entity: &T) -> QueryResult<T>;

    fn delete(&self, id: K) -> QueryResult<usize>;
}

pub struct UserRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> Repository<User, Uuid> for UserRepository<'a> {

    fn new(conn: &mut PgConnection) -> Self {
        Self { conn }
    }

    fn create(&self, new_user: &User) -> QueryResult<User> {
        diesel::insert_into(users)
            .values(new_user)
            .get_result(self.conn)
    }

    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> IntoIter<User> {
        let limit = size.unwrap_or(10);
        let offset = page.unwrap_or(0) * limit;
        let results = users
            .limit(limit)
            .offset(offset)
            .load::<User>(self.conn)
            .expect("Error loading users");

        results.into_iter().collect::<Vec<_>>().into_iter()
    }

    fn find_one_by_id(&self, user_id: Uuid) -> QueryResult<User> {
        users.filter(external_id.eq(user_id))
            .first(self.conn)
    }

    // fn update(&self, updated_user: &User) -> QueryResult<User> {
    //     diesel::update(users.filter(updated_user.external_id))
    //         .set()
    //         .get_result(self.conn)
    // }

    fn delete(&self, user_id: Uuid) -> QueryResult<usize> {
        diesel::delete(users.filter(external_id.eq(user_id)))
            .execute(self.conn)
    }
}