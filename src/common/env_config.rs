#[derive(Clone, PartialEq)]
pub enum Env {
    Local,
    Dev,
    Prod,
}

#[derive(Clone)]
pub struct DataWarehouseConfig {
    pub host: String,
    pub port: String,
    pub admin_creds: DataWarehouseCreds,
}

#[derive(Clone)]
pub struct DataWarehouseCreds {
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DataWarehouseCreds {
    pub fn new(user: String, password: String, database: String) -> DataWarehouseCreds {
        DataWarehouseCreds {
            user,
            password,
            database,
        }
    }

    pub fn construct_from_env() -> DataWarehouseCreds {
        let user = std::env::var("DATA_WAREHOUSE_USER").expect("DATA_WAREHOUSE_USER missing");
        let password =
            std::env::var("DATA_WAREHOUSE_PASSWORD").expect("DATA_WAREHOUSE_PASSWORD missing");
        let database = std::env::var("DATA_WAREHOUSE_DB").expect("DATA_WAREHOUSE_DB missing");
        DataWarehouseCreds {
            user,
            password,
            database,
        }
    }
}

impl DataWarehouseConfig {
    pub fn construct_from_env() -> DataWarehouseConfig {
        let host = std::env::var("DATA_WAREHOUSE_HOST").expect("DATA_WAREHOUSE_HOST missing");
        let port = std::env::var("DATA_WAREHOUSE_PORT").expect("DATA_WAREHOUSE_PORT missing");
        let admin_creds = DataWarehouseCreds::construct_from_env();
        DataWarehouseConfig {
            host,
            port,
            admin_creds,
        }
    }
    pub fn get_url_admin(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.admin_creds.user,
            self.admin_creds.password,
            self.host,
            self.port,
            self.admin_creds.database
        )
    }

    pub fn get_url_creds(&self, creds: DataWarehouseCreds) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            creds.user, creds.password, self.host, self.port, creds.database
        )
    }
}

#[derive(Clone)]
pub struct AirbyteConfig {
    pub url_base: String,
    pub user: String,
    pub password: String,
}

impl AirbyteConfig {
    pub fn construct_from_env() -> AirbyteConfig {
        let url_base = std::env::var("AIRBYTE_URL_BASE").expect("AIRBYTE_URL_BASE missing");
        let user = std::env::var("AIRBYTE_USER").expect("AIRBYTE_USER missing");
        let password = std::env::var("AIRBYTE_PASSWORD").expect("AIRBYTE_PASSWORD missing");
        AirbyteConfig {
            url_base,
            user,
            password,
        }
    }
}

#[derive(Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn construct_from_env() -> DatabaseConfig {
        let user = std::env::var("DATABASE_USER").expect("DATABASE_USER missing");
        let password = std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD missing");
        let database = std::env::var("DATABASE_DB").expect("DATABASE_DB missing");

        let host = std::env::var("DATABASE_HOST").expect("DATABASE_HOST missing");
        let port = std::env::var("DATABASE_PORT").expect("DATABASE_PORT missing");

        DatabaseConfig {
            host,
            port,
            user,
            password,
            database,
        }
    }

    pub fn get_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Clone)]
pub struct EnvConfig {
    pub env: Env,
    pub database_reader: DatabaseConfig,
    pub database_writer: DatabaseConfig,
    pub cookie_secret: String,
    pub airbyte: AirbyteConfig,
    pub data_warehouse: DataWarehouseConfig,
}

impl EnvConfig {
    pub fn new() -> Self {
        if let Err(_) = std::env::var("SEEABLE_ENV") {
            dotenvy::from_filename(".env").ok();
        }

        let env = std::env::var("SEEABLE_ENV").expect("SEEABLE_ENV missing");
        let database_reader = DatabaseConfig::construct_from_env();
        let database_writer = DatabaseConfig::construct_from_env();
        let cookie_secret = std::env::var("COOKIE_SECRET").expect("COOKIE_SECRET missing");

        let env = match env.as_str() {
            "local" => Env::Local,
            "dev" => Env::Dev,
            "prod" => Env::Prod,
            _ => panic!("Invalid environment"),
        };

        let data_warehouse = DataWarehouseConfig::construct_from_env();

        let airbyte = AirbyteConfig::construct_from_env();

        Self {
            env,
            database_reader,
            database_writer,
            cookie_secret,
            airbyte,
            data_warehouse,
        }
    }
}
