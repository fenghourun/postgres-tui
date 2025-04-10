use deadpool_postgres::{Config as PoolConfig, ManagerConfig, Pool, RecyclingMethod, Runtime};
use std::collections::HashMap;
use std::error::Error;
use tokio_postgres::{Config as PgConfig, NoTls};

#[derive(Clone, Hash, PartialEq, Eq)]
struct ConnectionContext {
    host: String,
    port: u16,
    dbname: String,
    user: String,
}

impl ConnectionContext {
    pub fn to_pg_config(&self) -> PgConfig {
        let mut config = PgConfig::new();

        config.host(&self.host);
        config.port(self.port);
        config.dbname(&self.dbname);
        config.user(&self.user);

        config
    }
}

pub struct ConnectionPoolManager {
    pools: HashMap<ConnectionContext, Pool>,
}

impl ConnectionPoolManager {
    pub fn new() -> Self {
        ConnectionPoolManager {
            pools: HashMap::new(),
        }
    }

    pub async fn get_or_create_pool(
        &mut self,
        host: &str,
        port: u16,
        dbname: &str,
        user: &str,
    ) -> Result<Pool, Box<dyn Error>> {
        let context = ConnectionContext {
            host: host.to_string(),
            port,
            dbname: dbname.to_string(),
            user: user.to_string(),
        };

        if let Some(pool) = self.pools.get(&context) {
            return Ok(pool.clone());
        }

        let mut pool_config = PoolConfig::new();
        pool_config.dbname = Some(dbname.to_string());
        pool_config.user = Some(user.to_string());
        pool_config.port = Some(port);
        pool_config.host = Some(host.to_string());
        pool_config.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = pool_config.create_pool(Some(Runtime::Tokio1), NoTls)?;
        self.pools.insert(context, pool.clone());

        Ok(pool)
    }
}
