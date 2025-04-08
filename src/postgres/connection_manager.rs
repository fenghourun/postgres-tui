use tokio_postgres::{Client, Error, NoTls, Row};

use crate::app::PSQLConnectionOptions;
use cli_log::error;

pub struct ConnectionManager {
    client: Client,
    connection_options: PSQLConnectionOptions,
}

impl ConnectionManager {
    pub async fn new(connection_options: PSQLConnectionOptions) -> Result<(), Error> {
        let (client, connection) = tokio_postgres::connect(
            format!(
                "host={} user={} dbname={}",
                connection_options.host, connection_options.user, connection_options.db_name,
            )
            .as_str(),
            NoTls,
        )
        .await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                error!("Connection error: {}", e);
            }
        });

        Ok(())
    }

    pub fn get_databases(&mut self) -> Vec<Row> {
        self.client
            .query(
                "SELECT datname from pg_database WHERE datistemplate = false",
                &[],
            )
            .expect("Get databases")
    }

    pub async fn get_tables_for_database(&mut self) -> Result<Vec<Row>, Error> {
        self.client
            .query(
                "SELECT tablename FROM pg_tables where schemaname = 'public'",
                &[],
            )
            .await?;
    }

    pub fn create_database_connection(
        &mut self,
        connection_options: PSQLConnectionOptions,
    ) -> Result<(), Error> {
        let client_result = Client::connect(
            format!(
                "host={} user={} dbname={}",
                connection_options.host, connection_options.user, connection_options.db_name
            )
            .as_str(),
            NoTls,
        );

        match client_result {
            Ok(client) => {
                self.client = client;
                self.connection_options = connection_options;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn get_table(&mut self, table_name: &String) -> Result<Vec<Row>, Error> {
        let result = self.client.query(
            "SELECT column_name FROM information_schema.columns where table_name = ($1)",
            &[&table_name],
        );

        result
    }

    pub fn get_data(&mut self, table_name: &String) -> Result<Vec<Row>, Error> {
        self.client
            .query(&format!("SELECT * FROM {} LIMIT 10", table_name), &[])
    }
}
