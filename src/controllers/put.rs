use crate::{
    constants::{RESPONSE_INTERNAL_SERVER_ERROR, RESPONSE_NOT_FOUND, RESPONSE_OK},
    controllers::{get_id, get_user_request_body},
    database::Database,
};

pub struct PutController;

impl PutController {
    pub fn new() -> Self {
        Self
    }

    pub fn put(&self, request: &str, database: &mut Database) -> (String, String) {
        match (get_id(&request), get_user_request_body(&request)) {
            (Ok(id), Ok(user)) => {
                let rows_affected = database
                    .get_client()
                    .execute(
                        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                        &[&user.name, &user.email, &id],
                    )
                    .unwrap();

                if rows_affected == 0 {
                    return (RESPONSE_NOT_FOUND.to_string(), "User Not Found".to_string());
                }

                (RESPONSE_OK.to_string(), "User updated".to_string())
            }
            _ => (
                RESPONSE_INTERNAL_SERVER_ERROR.to_string(),
                "Internal Server Error".to_string(),
            ),
        }
    }
}
