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
        "GetTweet" => {
            return response::Response {
                operation: Some(response::response::Operation::GetTweet(
                    response::GetTweet {
                        status: status as i32,
                        message: None,
                        tweet: None,
                    },
                )),
            };
        }
        "ListTweets" => {
            return response::Response {
                operation: Some(response::response::Operation::ListTweets(
                    response::ListTweets {
                        status: status as i32,
                        message: None,
                        tweets: Vec::new(),
                    },
                )),
            };
        }

        "UpdateTweet" => {
            return response::Response {
                operation: Some(response::response::Operation::UpdateTweet(
                    response::UpdateTweet {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "ReactToTweet" => {
            return response::Response {
                operation: Some(response::response::Operation::ReactToTweet(
                    response::ReactToTweet {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "UndoTweetReact" => {
            return response::Response {
                operation: Some(response::response::Operation::UndoReactToTweet(
                    response::UndoReactToTweet {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "RepostTweet" => {
            return response::Response {
                operation: Some(response::response::Operation::RepostTweet(
                    response::RepostTweet {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "ReplyToTweet" => {
            return response::Response {
                operation: Some(response::response::Operation::ReplyToTweet(
                    response::ReplyToTweet {
                        status: status as i32,
                        message: None,
                        reply_id: Vec::new(),
                    },
                )),
            };
        }

        "EditReply" => {
            return response::Response {
                operation: Some(response::response::Operation::EditReply(
                    response::EditReply {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "GetReply" => {
            return response::Response {
                operation: Some(response::response::Operation::GetReply(
                    response::GetReply {
                        status: status as i32,
                        message: None,
                        reply: None,
                    },
                )),
            };
        }

        "ListReplies" => {
            return response::Response {
                operation: Some(response::response::Operation::ListReplies(
                    response::ListReplies {
                        status: status as i32,
                        message: None,
                        replies: Vec::new(),
                    },
                )),
            };
        }

        "RemoveReply" => {
            return response::Response {
                operation: Some(response::response::Operation::RemoveReply(
                    response::RemoveReply {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "CreateOneToOneChat" => {
            return response::Response {
                operation: Some(response::response::Operation::CreateOneToOneChat(
                    response::CreateOneToOneChat {
                        status: status as i32,
                        message: None,
                        chat_id: Vec::new(),
                    },
                )),
            };
        }

        "CreateGroup" => {
            return response::Response {
                operation: Some(response::response::Operation::CreateGroup(
                    response::CreateGroup {
                        status: status as i32,
                        message: None,
                        chat_id: Vec::new(),
                    },
                )),
            };
        }

        "UpdateGroup" => {
            return response::Response {
                operation: Some(response::response::Operation::UpdateGroup(
                    response::UpdateGroup {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }
        "ListGroups" => {
            return response::Response {
                operation: Some(response::response::Operation::ListGroups(
                    response::ListGroups {
                        status: status as i32,
                        message: None,
                        groups: Vec::new(),
                    },
                )),
            };
        }
        "AddUserToGroup" => {
            return response::Response {
                operation: Some(response::response::Operation::AddUserToGroup(
                    response::AddUserToGroup {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "RemoveGroup" => {
            return response::Response {
                operation: Some(response::response::Operation::RemoveGroup(
                    response::RemoveGroup {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "ExitFromGroup" => {
            return response::Response {
                operation: Some(response::response::Operation::ExitFromGroup(
                    response::ExitFromGroup {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "GetChat" => {
            return response::Response {
                operation: Some(response::response::Operation::GetChat(response::GetChat {
                    status: status as i32,
                    message: None,
                    chat: None,
                })),
            };
        }

        "RemoveUserFromGroup" => {
            return response::Response {
                operation: Some(response::response::Operation::RemoveUserFromGroup(
                    response::RemoveUserFromGroup {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }
        "ClearChat" => {
            return response::Response {
                operation: Some(response::response::Operation::ClearChat(
                    response::ClearChat {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }
        "SendMessage" => {
            return response::Response {
                operation: Some(response::response::Operation::SendMessage(
                    response::SendMessage {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "RemoveMessage" => {
            return response::Response {
                operation: Some(response::response::Operation::RemoveMessage(
                    response::RemoveMessage {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "EditMessage" => {
            return response::Response {
                operation: Some(response::response::Operation::EditMessage(
                    response::EditMessage {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "ListChat" => {
            return response::Response {
                operation: Some(response::response::Operation::ListChat(
                    response::ListChat {
                        status: status as i32,
                        message: None,
                        messages: Vec::new(),
                    },
                )),
            };
        }

        "ReadMessage" => {
            return response::Response {
                operation: Some(response::response::Operation::ReadMessage(
                    response::ReadMessage {
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
