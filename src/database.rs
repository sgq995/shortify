use diesel::{r2d2, sqlite::SqliteConnection};
use dotenvy::dotenv;

pub type DatabasePool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub fn connect() -> DatabasePool {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("DATABASE_URL should be valid path to SQLite file")
}
