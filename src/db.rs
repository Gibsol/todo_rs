use crate::models::Todo;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, Error, NoTls};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn connect() -> Result<Self, Error> {
        dotenv().ok();

        let db_host = env::var("DB_HOST").expect("DB_HOST not set");
        let db_user = env::var("DB_USER").expect("DB_USER not set");
        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
        let db_name = env::var("DB_NAME").expect("DB_NAME not set");

        let connection_string = format!(
            "host={} user={} password={} dbname={}",
            db_host, db_user, db_password, db_name
        );

        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Database { client })
    }

    pub async fn get_tasks(&self) -> Result<Vec<Todo>, Error> {
        let rows = self.client.query("SELECT * FROM todos", &[]).await?;

        let mut tasks: Vec<Todo> = Vec::new();
        for row in rows {
            let id: i64 = row.get(0);
            let title: String = row.get(1);
            let description: String = row.get(2);
            let is_done: bool = row.get(3);

            let todo = Todo {
                id,
                title,
                description,
                is_done,
            };

            tasks.push(todo);
        }

        Ok(tasks)
    }
}
