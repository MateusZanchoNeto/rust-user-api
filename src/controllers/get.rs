use crate::{
    constants::{RESPONSE_INTERNAL_SERVER_ERROR, RESPONSE_NOT_FOUND, RESPONSE_OK},
    controllers::get_id,
    database::Database,
    user::User,
};

pub struct GetController;

impl GetController {
    pub fn new() -> Self {
        GetController
    }

    pub fn get(&self, request: &str, database: &mut Database) -> (String, String) {
        match get_id(&request) {
            Ok(id) => {
                match database
                    .get_client()
                    .query_one("SELECT * FROM users WHERE id = $1", &[&id])
                {
                    Ok(user) => {
                        let user = User {
                            id: user.get(0),
                            name: user.get(1),
                            email: user.get(2),
                        };

                        (
                            RESPONSE_OK.to_string(),
                            serde_json::to_string(&user).unwrap(),
                        )
                    }
                    _ => (RESPONSE_NOT_FOUND.to_string(), "User Not Found".to_string()),
                }
            }
            _ => (
                RESPONSE_INTERNAL_SERVER_ERROR.to_string(),
                "Internal Server Error".to_string(),
            ),
        }
    }

    pub fn get_all(&self, database: &mut Database) -> (String, String) {
        let mut users = vec![];

        for row in database.client.query("SELECT * FROM users", &[]).unwrap() {
            users.push(User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
            });
        }

        (
            RESPONSE_OK.to_string(),
            serde_json::to_string(&users).unwrap(),
        )
    }
}
