// To get the enviroment variables DATABASE_URL (used to connect to the database) and HOST (used to start the API server).
use std::env;

// DATABASE_URL
pub const DB_URL: &str = env!("DATABASE_URL");

// HOST
pub const HOST: &str = env!("HOST");

// Responses used in the controllers
pub const RESPONSE_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
pub const RESPONSE_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
pub const RESPONSE_INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
