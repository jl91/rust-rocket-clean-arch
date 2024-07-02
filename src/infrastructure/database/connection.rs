use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use r2d2::Pool;

pub struct ConnectionFactoryImpl;

pub trait ConnectionFactory {
    fn connect(&self) -> Pool<ConnectionManager<PgConnection>>;
}

impl ConnectionFactoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl ConnectionFactory for ConnectionFactoryImpl{

    fn connect(&self) -> Pool<ConnectionManager<PgConnection>> {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    }

}