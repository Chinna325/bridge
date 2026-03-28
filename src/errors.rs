use crate::response::{self, Response};

pub async fn form_response(operation: &str, status: response::Status) -> Response {
    match operation {
        "AddUser" => {
            return response::Response {
                operation: Some(response::response::Operation::AddUser(response::AddUser {
                    status: status as i32,
                    message: None,
                })),
            };
        }
        "VerifyUser" => {
            return response::Response {
                operation: Some(response::response::Operation::VerifyUser(
                    response::VerifyUser {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }
        _ => {
            panic!("Invalid operation");
        }
    }
}
