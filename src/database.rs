mod database {
    use sqlx::{Pool, Postgres};
    use std::env;
    use log::{info, error};

    pub struct Config {
        pub database_url: String,
    }

    pub async fn initialize_database_connection(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
        let pool = Pool::<Postgres>::connect(&config.database_url).await;
        match pool {
            Ok(_) => {
                info!("Database connection established successfully.");
                pool
            },
            Err(e) => {
                error!("Failed to connect to the database: {}", e);
                Err(e)
            }
        }
    }
}