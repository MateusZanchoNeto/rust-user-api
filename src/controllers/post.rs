use crate::{
    constants::{RESPONSE_INTERNAL_SERVER_ERROR, RESPONSE_OK},
    controllers::get_user_request_body,
    database::Database,
};

pub struct PostController;

impl PostController {
    pub fn new() -> Self {
        Self
    }

    pub fn post(&self, request: &str, database: &mut Database) -> (String, String) {
        match get_user_request_body(&request) {
            Ok(user) => {
                database
                    .get_client()
                    .execute(
                        "INSERT INTO users (name, email) VALUES ($1, $2)",
                        &[&user.name, &user.email],
                    )
                    .unwrap();
                (RESPONSE_OK.to_string(), "User created".to_string())
            }
            _ => (
                RESPONSE_INTERNAL_SERVER_ERROR.to_string(),
                "Internal Server Error".to_string(),
            ),
        }
    }
}
