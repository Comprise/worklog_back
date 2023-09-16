#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_address: String,
    pub front_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub secret_key: String,
    pub jwt_access_exp: i64,
    pub jwt_refresh_exp: i64,
    pub org_id: String
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL should be set");
        let server_address = std::env::var("SERVER_ADDRESS")
            .expect("SERVER_ADDRESS should be set");
        let front_url = std::env::var("FRONT_URL")
            .expect("FRONT_URL should be set");
        let client_id = std::env::var("CLIENT_ID")
            .expect("CLIENT_ID should be set");
        let client_secret = std::env::var("CLIENT_SECRET")
            .expect("CLIENT_SECRET should be set");
        let secret_key = std::env::var("SECRET_KEY")
            .expect("SECRET_KEY should be set");
        let jwt_access_exp = std::env::var("JWT_ACCESS_EXP")
            .expect("JWT_ACCESS_EXP should be set");
        let jwt_refresh_exp = std::env::var("JWT_REFRESH_EXP")
            .expect("JWT_REFRESH_EXP should be set");
        let org_id = std::env::var("ORG_ID")
            .expect("ORG_ID should be set");
        Config {
            database_url,
            server_address,
            front_url,
            client_id,
            client_secret,
            secret_key,
            jwt_access_exp: jwt_access_exp.parse::<i64>()
                .expect("JWT_ACCESS_EXP does not match the type"),
            jwt_refresh_exp: jwt_refresh_exp.parse::<i64>()
                .expect("JWT_REFRESH_EXP does not match the type"),
            org_id
        }
    }
}