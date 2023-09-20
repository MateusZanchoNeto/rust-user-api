use crate::constants::DB_URL;
use postgres::{Client, Error as PostgresError, NoTls};

// Database struct used to connect to the database
pub struct Database {
    pub client: Client,
}

impl Database {
    pub fn new() -> Result<Self, PostgresError> {
        let client = Client::connect(DB_URL, NoTls)?;
        Ok(Database { client })
    }

    pub fn set_database(&mut self) -> Result<(), PostgresError> {
        self.client.batch_execute(
            "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )",
        )?;
        Ok(())
    }

    pub fn get_client(&mut self) -> &mut Client {
        &mut self.client
    }
}
