use crate::{
    constants::RESPONSE_NOT_FOUND,
    controllers::{
        delete::DeleteController, get::GetController, post::PostController, put::PutController,
    },
    database::Database,
    user::User,
};
use std::{borrow::BorrowMut, io::Write, net::TcpStream};

mod delete;
mod get;
mod post;
mod put;

pub struct Controllers {
    get: GetController,
    post: PostController,
    put: PutController,
    delete: DeleteController,
}

impl Controllers {
    pub fn new() -> Self {
        Controllers {
            get: GetController::new(),
            post: PostController::new(),
            put: PutController::new(),
            delete: DeleteController::new(),
        }
    }

    pub fn handle(&mut self, mut stream: TcpStream, request: String) {
        let (status_line, content) = match &*request {
            r if r.starts_with("POST /users") => {
                self.post.post(r, Database::new().unwrap().borrow_mut())
            }
            r if r.starts_with("GET /users/") => {
                self.get.get(r, Database::new().unwrap().borrow_mut())
            }
            r if r.starts_with("GET /users") => {
                self.get.get_all(Database::new().unwrap().borrow_mut())
            }
            r if r.starts_with("PUT /users/") => {
                self.put.put(r, Database::new().unwrap().borrow_mut())
            }
            r if r.starts_with("DELETE /users/") => {
                self.delete.delete(r, Database::new().unwrap().borrow_mut())
            }
            _ => (RESPONSE_NOT_FOUND.to_string(), "Not Found".to_string()),
        };

        stream
            .write_all(format!("{}{}", status_line, content).as_bytes())
            .unwrap();
    }
}

pub fn get_id(request: &str) -> Result<i32, std::num::ParseIntError> {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .parse::<i32>()
}

pub fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
