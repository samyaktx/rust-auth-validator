#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")?;
        let jwt_secret = std::env::var("JWT_SECRET")?;
        let jwt_maxage = std::env::var("JWT_MAXAGE")?;
        let port = std::env::var("PORT")?;

        let config = Self {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i64>()?,
            port: port.parse::<u16>()?,
        };

        Ok(config)
    }
}
