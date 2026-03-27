use prost::Message;

use crate::{request, response::Response};

pub async fn process_request(data: Vec<u8>) -> Option<Vec<u8>> {
    let req = request::Request::decode(data.as_slice()).ok()?;
    let resp = req.handle().await?;
    Some(resp.encode_to_vec())
}

impl request::Request {
    pub async fn handle(&self) -> Option<Response> {
        match &self.operation {
            Some(request::request::Operation::AddUser(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::VerifyUser(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::RemoveUser(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::AddTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ChangePassword(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::GetProfilePicture(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::GetUser(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UpdateUser(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::RemoveProfilePicture(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::RemoveReply(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::RemoveTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::GetTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::SignIn(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::SignOut(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::Follow(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UnFollow(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ListFollowers(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ListReplies(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::RepostTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ListTweets(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ReplyToTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ReactToTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UndoReactToTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UpdateTweet(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::EditReply(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::GetReply(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::UpdateProfilePicture(req)) => {
                return req.handle().await;
            }
            Some(request::request::Operation::ResetPassword(req)) => {
                return req.handle().await;
            }
            _ => return None,
        }
    }
}

impl request::AddUser {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::VerifyUser {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::RemoveUser {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::AddTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ChangePassword {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::GetProfilePicture {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::GetUser {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UpdateUser {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::RemoveReply {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::RemoveTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::GetTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::SignIn {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::SignOut {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::Follow {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UnFollow {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ListFollowers {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ListReplies {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::RepostTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ListTweets {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ReplyToTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ReactToTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UndoReactToTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UpdateTweet {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::EditReply {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::GetReply {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::UploadProfilePicture {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::ResetPassword {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}

impl request::RemoveProfilePicture {
    pub async fn handle(&self) -> Option<Response> {
        todo!()
    }
}
