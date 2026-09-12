use crate::operations::traits::RequestHandler;
use crate::{Context, protos::request, protos::response::Response};
use jbackend_runtime::TWServer;

impl request::Request {
    pub async fn handle(&self, backend: &TWServer, ctx: &mut Context) -> Result<Response, ()> {
        match &self.operation {
            Some(request::request::Operation::AddUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::VerifyUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemoveUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::AddPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ChangePassword(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetProfilePicture(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UpdateUser(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemoveProfilePicture(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemoveReply(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemovePost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::SignIn(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::SignOut(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::Follow(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UnFollow(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListFollowers(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListReplies(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RepostPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListPosts(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ReplyToPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ReactToPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UndoReactToPost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UpdatePost(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::EditReply(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetReply(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UpdateProfilePicture(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ResetPassword(req)) => {
                return req.handle(backend, ctx).await;
            }

            Some(request::request::Operation::CreateOneToOneConversation(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::CreateGroup(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::UpdateGroup(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListGroups(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::AddUserToGroup(req)) => {
                return req.handle(backend, ctx).await;
            }

            Some(request::request::Operation::RemoveUserFromGroup(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ExitFromGroup(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetConversation(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ClearConversation(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::SendMessage(req)) => {
                return req.handle(backend, ctx).await;
            }

            Some(request::request::Operation::RemoveMessage(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::EditMessage(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListConversation(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ReadMessage(req)) => {
                return req.handle(backend, ctx).await;
            }

            Some(request::request::Operation::AddAttachment(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListAttachements(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemoveAttachement(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ReadAttchment(req)) => {
                return req.handle(backend, ctx).await;
            }

            Some(request::request::Operation::AddStory(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::GetStory(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::RemoveStory(req)) => {
                return req.handle(backend, ctx).await;
            }
            Some(request::request::Operation::ListStories(req)) => {
                return req.handle(backend, ctx).await;
            }

            _ => return Err(()),
        }
    }
}
