use crate::response::{self, Response};

pub async fn form_response(operation: &str, _status: response::Status) -> Response {
    match operation {
        "AddUser" => {
            return response::Response {
                operation: Some(response::response::Operation::AddUser(response::AddUser {})),
            };
        }
        _ => {
            panic!("Invalid operation");
        }
    }
}
