use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use r2d2::Pool;

pub struct ConnectionFactoryImpl{
    database_url: String,
}

pub trait ConnectionFactory {
    fn connect(&self) -> Pool<ConnectionManager<PgConnection>>;
}

impl ConnectionFactoryImpl {
    pub fn new(
        database_url: String
    ) -> Self {
        Self{
            database_url
        }
    }
}

impl ConnectionFactory for ConnectionFactoryImpl{

    fn connect(&self) -> Pool<ConnectionManager<PgConnection>> {
        dotenvy::dotenv().ok();
        let manager = ConnectionManager::<PgConnection>::new(
            &self.database_url
        );

        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    }

}