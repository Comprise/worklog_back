use diesel::r2d2;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn initialize_db_pool(database_url: &str) -> DbPool {
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

pub fn run_migrations(conn: DbPool) {
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    conn.get().unwrap().run_pending_migrations(MIGRATIONS).unwrap();
}
