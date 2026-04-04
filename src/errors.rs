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
        "SignIn" => {
            return response::Response {
                operation: Some(response::response::Operation::SignIn(response::SignIn {
                    status: status as i32,
                    message: None,
                })),
            };
        }
        "RemoveUser" => {
            return response::Response {
                operation: Some(response::response::Operation::RemoveUser(
                    response::RemoveUser {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }
        "AddTweet" => {
            return response::Response {
                operation: Some(response::response::Operation::AddTweet(
                    response::AddTweet {
                        status: status as i32,
                        message: None,
                        tweet_id: Vec::new(),
                    },
                )),
            };
        }
        "RemoveTweet" => {
            return response::Response {
                operation: Some(response::response::Operation::RemoveTweet(
                    response::RemoveTweet {
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
