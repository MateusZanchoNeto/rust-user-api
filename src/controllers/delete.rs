use crate::{
    constants::{RESPONSE_INTERNAL_SERVER_ERROR, RESPONSE_NOT_FOUND, RESPONSE_OK},
    controllers::get_id,
    database::Database,
};

pub struct DeleteController;

impl DeleteController {
    pub fn new() -> Self {
        Self
    }

    pub fn delete(&self, request: &str, database: &mut Database) -> (String, String) {
        match get_id(&request) {
            Ok(id) => {
                let rows_affected = database
                    .get_client()
                    .execute("DELETE FROM users WHERE id = $1", &[&id])
                    .unwrap();

                if rows_affected == 0 {
                    return (RESPONSE_NOT_FOUND.to_string(), "User Not Found".to_string());
                }

                (RESPONSE_OK.to_string(), "User deleted".to_string())
            }
            _ => (
                RESPONSE_INTERNAL_SERVER_ERROR.to_string(),
                "Internal Server Error".to_string(),
            ),
        }
    }
}
