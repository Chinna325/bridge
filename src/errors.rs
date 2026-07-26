use crate::protos::response::{self, Response};

pub async fn form_response(operation: &str, status: response::Status) -> Response {
    match operation {
        "AddUser" => {
            return response::Response {
                operation: Some(response::response::Operation::AddUser(response::AddUser {
                    status: status as i32,
                    message: None,
                    otp: String::new(),
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
        "AddPost" => {
            return response::Response {
                operation: Some(response::response::Operation::AddPost(response::AddPost {
                    status: status as i32,
                    message: None,
                    post_id: Vec::new(),
                })),
            };
        }
        "RemovePost" => {
            return response::Response {
                operation: Some(response::response::Operation::RemovePost(
                    response::RemovePost {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }
        "GetPost" => {
            return response::Response {
                operation: Some(response::response::Operation::GetPost(response::GetPost {
                    status: status as i32,
                    message: None,
                    post: None,
                })),
            };
        }
        "ListPosts" => {
            return response::Response {
                operation: Some(response::response::Operation::ListPosts(
                    response::ListPosts {
                        status: status as i32,
                        message: None,
                        posts: Vec::new(),
                    },
                )),
            };
        }

        "UpdatePost" => {
            return response::Response {
                operation: Some(response::response::Operation::UpdatePost(
                    response::UpdatePost {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "ReactToPost" => {
            return response::Response {
                operation: Some(response::response::Operation::ReactToPost(
                    response::ReactToPost {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "UndoPostReact" => {
            return response::Response {
                operation: Some(response::response::Operation::UndoReactToPost(
                    response::UndoReactToPost {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "ListFollowers" => {
            return response::Response {
                operation: Some(response::response::Operation::ListFollowers(
                    response::ListFollowers {
                        status: status as i32,
                        message: None,
                        user_emails: Vec::new(),
                    },
                )),
            };
        }
        "RepostPost" => {
            return response::Response {
                operation: Some(response::response::Operation::RepostPost(
                    response::RepostPost {
                        status: status as i32,
                        message: None,
                    },
                )),
            };
        }

        "ReplyToPost" => {
            return response::Response {
                operation: Some(response::response::Operation::ReplyToPost(
                    response::ReplyToPost {
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

        "CreateOneToOneConversation" => {
            return response::Response {
                operation: Some(response::response::Operation::CreateOneToOneConversation(
                    response::CreateOneToOneConversation {
                        status: status as i32,
                        message: None,
                        conversation_id: Vec::new(),
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
                        conversation_id: Vec::new(),
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

        "GetConversation" => {
            return response::Response {
                operation: Some(response::response::Operation::GetConversation(
                    response::GetConversation {
                        status: status as i32,
                        message: None,
                        conversation: None,
                    },
                )),
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
        "ClearConversation" => {
            return response::Response {
                operation: Some(response::response::Operation::ClearConversation(
                    response::ClearConversation {
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

        "ListConversation" => {
            return response::Response {
                operation: Some(response::response::Operation::ListConversation(
                    response::ListConversation {
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

        "Follow" => {
            return response::Response {
                operation: Some(response::response::Operation::Follow(response::Follow {
                    status: status as i32,
                    message: None,
                })),
            };
        }
        "UnFollow" => {
            return response::Response {
                operation: Some(response::response::Operation::UnFollow(
                    response::UnFollow {
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
