use deadpool_postgres::Pool;
use std::collections::HashMap;
use tokio_postgres::{connect, Client, Error, NoTls, Row};

use crate::app::PSQLConnectionOptions;
use cli_log::{error, info};

pub struct ConnectionManager {
    pools: HashMap<String, Pool>,
    configs: HashMap<String, PSQLConnectionOptions>,
    current_connection: String,
    client: Client,
}

impl ConnectionManager {
    pub async fn new(
        connection_options: PSQLConnectionOptions,
    ) -> Result<ConnectionManager, Error> {
        let (client, connection) = connect(
            format!(
                "host={} user={} dbname={}",
                connection_options.host, connection_options.user, connection_options.db_name,
            )
            .as_str(),
            NoTls,
        )
        .await?;

        info!("Connected to database");

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                error!("Connection error: {}", e);
            }
        });

        Ok(ConnectionManager {
            client,
            current_connection: String::new(),
            pools: HashMap::new(),
            configs: HashMap::new(),
        })
    }

    pub async fn get_databases(&mut self) -> Result<Vec<Row>, Error> {
        let databases = self
            .client
            .query(
                "SELECT datname from pg_database WHERE datistemplate = false",
                &[],
            )
            .await?;

        Ok(databases)
    }

    pub async fn get_tables_for_database(&mut self) -> Result<Vec<Row>, Error> {
        let tables = self
            .client
            .query(
                "SELECT tablename FROM pg_tables where schemaname = 'public'",
                &[],
            )
            .await?;

        Ok(tables)
    }

    pub async fn create_database_connection(
        &mut self,
        connection_options: PSQLConnectionOptions,
    ) -> Result<(), Error> {
        connect(
            format!(
                "host={} user={} dbname={}",
                connection_options.host, connection_options.user, connection_options.db_name
            )
            .as_str(),
            NoTls,
        )
        .await?;

        Ok(())
    }

    pub async fn get_table(&mut self, table_name: &String) -> Result<Vec<Row>, Error> {
        self.client
            .query(
                "SELECT column_name FROM information_schema.columns where table_name = ($1)",
                &[&table_name],
            )
            .await
    }

    pub async fn get_data(&mut self, table_name: &String) -> Result<Vec<Row>, Error> {
        self.client
            .query(&format!("SELECT * FROM {} LIMIT 10", table_name), &[])
            .await
    }
}
