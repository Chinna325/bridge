use std::fmt::Display;

use crate::protos::{self, request, response, service_request, service_response};

impl Display for protos::request::Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operation = match &self.operation {
            Some(request::request::Operation::AddUser(_)) => "AddUser",
            Some(request::request::Operation::GetUser(_)) => "GetUser",
            Some(request::request::Operation::SignIn(_)) => "SignIn",
            Some(request::request::Operation::SignOut(_)) => "SignOut",
            Some(request::request::Operation::Follow(_)) => "Follow",
            Some(request::request::Operation::UnFollow(_)) => "UnFollow",
            Some(request::request::Operation::ListFollowers(_)) => "ListFollowers",
            Some(request::request::Operation::AddPost(_)) => "AddPost",
            Some(request::request::Operation::RemovePost(_)) => "RemovePost",
            Some(request::request::Operation::ListPosts(_)) => "ListPosts",
            Some(request::request::Operation::GetPost(_)) => "GetPost",
            Some(request::request::Operation::UpdatePost(_)) => "UpdatePost",
            Some(request::request::Operation::AddAttachment(_)) => "AddAttachment",
            Some(request::request::Operation::RemoveAttachement(_)) => "RemoveAttachement",
            Some(request::request::Operation::ListAttachements(_)) => "ListAttachements",
            Some(request::request::Operation::ReadAttchment(_)) => "ReadAttachment",
            Some(request::request::Operation::RepostPost(_)) => "RepostPost",
            Some(request::request::Operation::VerifyUser(_)) => "VerifyUser",
            Some(request::request::Operation::RemoveUser(_)) => "RemoveUser",
            Some(request::request::Operation::UpdateProfilePicture(_)) => "UpdateProfilePicture",
            Some(request::request::Operation::RemoveProfilePicture(_)) => "RemoveProfilePicture",
            Some(request::request::Operation::GetProfilePicture(_)) => "GetProfilePicture",
            Some(request::request::Operation::UpdateUser(_)) => "UpdateUser",
            Some(request::request::Operation::ChangePassword(_)) => "ChangePassword",
            Some(request::request::Operation::ResetPassword(_)) => "ResetPassword",
            Some(request::request::Operation::ReplyToPost(_)) => "ReplyToPost",
            Some(request::request::Operation::EditReply(_)) => "EditReply",
            Some(request::request::Operation::RemoveReply(_)) => "RemoveReply",
            Some(request::request::Operation::GetReply(_)) => "GetReply",
            Some(request::request::Operation::ListReplies(_)) => "ListReplies",
            Some(request::request::Operation::ReactToPost(_)) => "ReactToPost",
            Some(request::request::Operation::UndoReactToPost(_)) => "UndoReactToPost",
            Some(request::request::Operation::CreateOneToOneConversation(_)) => {
                "CreateOneToOneConversation"
            }
            Some(request::request::Operation::CreateGroup(_)) => "CreateGroup",
            Some(request::request::Operation::RemoveGroup(_)) => "RemoveGroup",
            Some(request::request::Operation::UpdateGroup(_)) => "UpdateGroup",
            Some(request::request::Operation::ListGroups(_)) => "ListGroups",
            Some(request::request::Operation::AddUserToGroup(_)) => "AddUserToGroup",
            Some(request::request::Operation::RemoveUserFromGroup(_)) => "RemoveUserFromGroup",
            Some(request::request::Operation::ExitFromGroup(_)) => "ExitFromGroup",
            Some(request::request::Operation::GetConversation(_)) => "GetConversation",
            Some(request::request::Operation::ClearConversation(_)) => "ClearConversation",
            Some(request::request::Operation::SendMessage(_)) => "SendMessage",
            Some(request::request::Operation::RemoveMessage(_)) => "RemoveMessage",
            Some(request::request::Operation::EditMessage(_)) => "EditMessage",
            Some(request::request::Operation::ListConversation(_)) => "ListConversation",
            Some(request::request::Operation::ReadMessage(_)) => "ReadMessage",
            Some(request::request::Operation::GetGroup(_)) => "GetGroup",
            Some(request::request::Operation::GetMessage(_)) => "GetMessage",
            _ => "Unknown",
        };
        let _ = write!(f, "Operation - {}", operation);
        Ok(())
    }
}

impl Display for protos::response::Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (operation, status) = match &self.operation {
            Some(response::response::Operation::AddUser(resp)) => ("AddUser", resp.status),
            Some(response::response::Operation::GetUser(resp)) => ("GetUser", resp.status),
            Some(response::response::Operation::SignIn(resp)) => ("SignIn", resp.status),
            Some(response::response::Operation::SignOut(resp)) => ("SignOut", resp.status),
            Some(response::response::Operation::Follow(resp)) => ("Follow", resp.status),
            Some(response::response::Operation::UnFollow(resp)) => ("UnFollow", resp.status),
            Some(response::response::Operation::ListFollowers(resp)) => {
                ("ListFollowers", resp.status)
            }
            Some(response::response::Operation::AddPost(resp)) => ("AddPost", resp.status),
            Some(response::response::Operation::RemovePost(resp)) => ("RemovePost", resp.status),
            Some(response::response::Operation::ListPosts(resp)) => ("ListPosts", resp.status),
            Some(response::response::Operation::GetPost(resp)) => ("GetPost", resp.status),
            Some(response::response::Operation::UpdatePost(resp)) => ("UpdatePost", resp.status),
            Some(response::response::Operation::AddAttachment(resp)) => {
                ("AddAttachment", resp.status)
            }
            Some(response::response::Operation::RemoveAttachement(resp)) => {
                ("RemoveAttachement", resp.status)
            }
            Some(response::response::Operation::ListAttachements(resp)) => {
                ("ListAttachements", resp.status)
            }
            Some(response::response::Operation::ReadAttchment(resp)) => {
                ("ReadAttachment", resp.status)
            }
            Some(response::response::Operation::RepostPost(resp)) => ("RepostPost", resp.status),
            Some(response::response::Operation::VerifyUser(resp)) => ("VerifyUser", resp.status),
            Some(response::response::Operation::RemoveUser(resp)) => ("RemoveUser", resp.status),
            Some(response::response::Operation::UpdateProfilePicture(resp)) => {
                ("UpdateProfilePicture", resp.status)
            }

            Some(response::response::Operation::RemoveProfilePicture(resp)) => {
                ("RemoveProfilePicture", resp.status)
            }
            Some(response::response::Operation::GetProfilePicture(resp)) => {
                ("GetProfilePicture", resp.status)
            }
            Some(response::response::Operation::UpdateUser(resp)) => ("UpdateUser", resp.status),
            Some(response::response::Operation::ChangePassword(resp)) => {
                ("ChangePassword", resp.status)
            }
            Some(response::response::Operation::ResetPassword(resp)) => {
                ("ResetPassword", resp.status)
            }
            Some(response::response::Operation::ReplyToPost(resp)) => ("ReplyToPost", resp.status),
            Some(response::response::Operation::EditReply(resp)) => ("EditReply", resp.status),
            Some(response::response::Operation::RemoveReply(resp)) => ("RemoveReply", resp.status),
            Some(response::response::Operation::GetReply(resp)) => ("GetReply", resp.status),
            Some(response::response::Operation::ListReplies(resp)) => ("ListReplies", resp.status),
            Some(response::response::Operation::ReactToPost(resp)) => ("ReactToPost", resp.status),
            Some(response::response::Operation::UndoReactToPost(resp)) => {
                ("UndoReactToPost", resp.status)
            }
            Some(response::response::Operation::CreateOneToOneConversation(resp)) => {
                ("CreateOneToOneConversation", resp.status)
            }
            Some(response::response::Operation::CreateGroup(resp)) => ("CreateGroup", resp.status),
            Some(response::response::Operation::RemoveGroup(resp)) => ("RemoveGroup", resp.status),
            Some(response::response::Operation::UpdateGroup(resp)) => ("UpdateGroup", resp.status),
            Some(response::response::Operation::ListGroups(resp)) => ("ListGroups", resp.status),
            Some(response::response::Operation::AddUserToGroup(resp)) => {
                ("AddUserToGroup", resp.status)
            }
            Some(response::response::Operation::RemoveUserFromGroup(resp)) => {
                ("RemoveUserFromGroup", resp.status)
            }
            Some(response::response::Operation::ExitFromGroup(resp)) => {
                ("ExitFromGroup", resp.status)
            }
            Some(response::response::Operation::GetConversation(resp)) => {
                ("GetConversation", resp.status)
            }
            Some(response::response::Operation::ClearConversation(resp)) => {
                ("ClearConversation", resp.status)
            }
            Some(response::response::Operation::SendMessage(resp)) => ("SendMessage", resp.status),
            Some(response::response::Operation::RemoveMessage(resp)) => {
                ("RemoveMessage", resp.status)
            }
            Some(response::response::Operation::EditMessage(resp)) => ("EditMessage", resp.status),
            Some(response::response::Operation::ListConversation(resp)) => {
                ("ListConversation", resp.status)
            }
            Some(response::response::Operation::ReadMessage(resp)) => ("ReadMessage", resp.status),
            Some(response::response::Operation::GetGroup(resp)) => ("GetGroup", resp.status),
            Some(response::response::Operation::GetMessage(resp)) => ("GetMessage", resp.status),
            _ => ("Unknown", -1),
        };
        let _ = write!(f, "Operation - {}  status - {}", operation, status);
        Ok(())
    }
}

impl Display for protos::service_request::ServiceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operation = match &self.operation {
            Some(service_request::service_request::Operation::AddUser(_)) => "AddUser",
            Some(service_request::service_request::Operation::CacheItem(_)) => "CacheItem",
            Some(service_request::service_request::Operation::GetCacheItem(_)) => "GetCacheItem",
            Some(service_request::service_request::Operation::ClearCache(_)) => "ClearCache",
            Some(service_request::service_request::Operation::AddToDb(_)) => "AddToDb",
            Some(service_request::service_request::Operation::GetUser(_)) => "GetUser",
            Some(service_request::service_request::Operation::RemoveUser(_)) => "RemoveUser",
            Some(service_request::service_request::Operation::UpdateUser(_)) => "UpdateUser",
            Some(service_request::service_request::Operation::SetProfilePicture(_)) => {
                "SetProfilePicture"
            }
            Some(service_request::service_request::Operation::GetProfilePicture(_)) => {
                "GetProfilePicture"
            }
            Some(service_request::service_request::Operation::RemoveProfilePicture(_)) => {
                "RemoveProfilePicture"
            }
            Some(service_request::service_request::Operation::AddPost(_)) => "AddPost",
            Some(service_request::service_request::Operation::GetPost(_)) => "GetPost",
            Some(service_request::service_request::Operation::RemovePost(_)) => "RemovePost",
            Some(service_request::service_request::Operation::ListPosts(_)) => "ListPosts",
            Some(service_request::service_request::Operation::UpdatePost(_)) => "UpdatePost",
            Some(service_request::service_request::Operation::PostReact(_)) => "PostReact",
            Some(service_request::service_request::Operation::AddReply(_)) => "AddReply",
            Some(service_request::service_request::Operation::UpdateReply(_)) => "UpdateReply",
            Some(service_request::service_request::Operation::RemoveReply(_)) => "RemoveReply",
            Some(service_request::service_request::Operation::GetReply(_)) => "GetReply",
            Some(service_request::service_request::Operation::ListReplies(_)) => "ListReplies",
            Some(service_request::service_request::Operation::CreateOneToOneConversation(_)) => {
                "CreateOneToOneConversation"
            }
            Some(service_request::service_request::Operation::CreateGroup(_)) => "CreateGroup",
            Some(service_request::service_request::Operation::RemoveGroup(_)) => "RemoveGroup",
            Some(service_request::service_request::Operation::UpdateGroup(_)) => "UpdateGroup",
            Some(service_request::service_request::Operation::ListGroups(_)) => "ListGroups",
            Some(service_request::service_request::Operation::AddUserToGroup(_)) => {
                "AddUserToGroup"
            }
            Some(service_request::service_request::Operation::RemoveUserFromGroup(_)) => {
                "RemoveUserFromGroup"
            }
            Some(service_request::service_request::Operation::ExitFromGroup(_)) => "ExitFromGroup",
            Some(service_request::service_request::Operation::GetConversation(_)) => {
                "GetConversation"
            }
            Some(service_request::service_request::Operation::ClearConversation(_)) => {
                "ClearConversation"
            }
            Some(service_request::service_request::Operation::SendMessage(_)) => "SendMessage",
            Some(service_request::service_request::Operation::RemoveMessage(_)) => "RemoveMessage",
            Some(service_request::service_request::Operation::EditMessage(_)) => "EditMessage",
            Some(service_request::service_request::Operation::ListConversation(_)) => {
                "ListConversation"
            }
            Some(service_request::service_request::Operation::ReadMessage(_)) => "ReadMessage",
            Some(service_request::service_request::Operation::GetMessage(_)) => "GetMessage",
            Some(service_request::service_request::Operation::GetGroup(_)) => "GetGroup",
            Some(service_request::service_request::Operation::Follow(_)) => "Follow",
            Some(service_request::service_request::Operation::UnFollow(_)) => "UnFollow",
            Some(service_request::service_request::Operation::ListFollowers(_)) => "ListFollowers",
            Some(service_request::service_request::Operation::AddAttachment(_)) => "AddAttachment",
            Some(service_request::service_request::Operation::RemoveAttachment(_)) => {
                "RemoveAttachement"
            }
            Some(service_request::service_request::Operation::ListAttachments(_)) => {
                "ListAttachements"
            }
            Some(service_request::service_request::Operation::ReadAttchment(_)) => "ReadAttachment",

            _ => "Unknown",
        };
        let _ = write!(f, "Operation - {}", operation);
        Ok(())
    }
}

impl Display for protos::service_response::ServiceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (operation, status) = match &self.operation {
            Some(service_response::service_response::Operation::AddUser(resp)) => {
                ("AddUser", resp.status)
            }
            Some(service_response::service_response::Operation::CacheItem(resp)) => {
                ("CacheItem", resp.status)
            }
            Some(service_response::service_response::Operation::GetCacheItem(resp)) => {
                ("GetCacheItem", resp.status)
            }
            Some(service_response::service_response::Operation::ClearCache(resp)) => {
                ("ClearCache", resp.status)
            }
            Some(service_response::service_response::Operation::AddToDb(resp)) => {
                ("AddToDb", resp.status)
            }
            Some(service_response::service_response::Operation::GetUser(resp)) => {
                ("GetUser", resp.status)
            }
            Some(service_response::service_response::Operation::RemoveUser(resp)) => {
                ("RemoveUser", resp.status)
            }
            Some(service_response::service_response::Operation::UpdateUser(resp)) => {
                ("UpdateUser", resp.status)
            }
            Some(service_response::service_response::Operation::SetProfilePicture(resp)) => {
                ("SetProfilePicture", resp.status)
            }
            Some(service_response::service_response::Operation::GetProfilePicture(resp)) => {
                ("GetProfilePicture", resp.status)
            }
            Some(service_response::service_response::Operation::RemoveProfilePicture(resp)) => {
                ("RemoveProfilePicture", resp.status)
            }
            Some(service_response::service_response::Operation::AddPost(resp)) => {
                ("AddPost", resp.status)
            }
            Some(service_response::service_response::Operation::GetPost(resp)) => {
                ("GetPost", resp.status)
            }
            Some(service_response::service_response::Operation::RemovePost(resp)) => {
                ("RemovePost", resp.status)
            }
            Some(service_response::service_response::Operation::ListPosts(resp)) => {
                ("ListPosts", resp.status)
            }
            Some(service_response::service_response::Operation::UpdatePost(resp)) => {
                ("UpdatePost", resp.status)
            }
            Some(service_response::service_response::Operation::PostReact(resp)) => {
                ("PostReact", resp.status)
            }
            Some(service_response::service_response::Operation::AddReply(resp)) => {
                ("AddReply", resp.status)
            }
            Some(service_response::service_response::Operation::UpdateReply(resp)) => {
                ("UpdateReply", resp.status)
            }
            Some(service_response::service_response::Operation::RemoveReply(resp)) => {
                ("RemoveReply", resp.status)
            }
            Some(service_response::service_response::Operation::GetReply(resp)) => {
                ("GetReply", resp.status)
            }
            Some(service_response::service_response::Operation::ListReplies(resp)) => {
                ("ListReplies", resp.status)
            }
            Some(service_response::service_response::Operation::CreateOneToOneConversation(
                resp,
            )) => ("CreateOneToOneConversation", resp.status),
            Some(service_response::service_response::Operation::CreateGroup(resp)) => {
                ("CreateGroup", resp.status)
            }
            Some(service_response::service_response::Operation::RemoveGroup(resp)) => {
                ("RemoveGroup", resp.status)
            }
            Some(service_response::service_response::Operation::UpdateGroup(resp)) => {
                ("UpdateGroup", resp.status)
            }
            Some(service_response::service_response::Operation::ListGroups(resp)) => {
                ("ListGroups", resp.status)
            }
            Some(service_response::service_response::Operation::AddUserToGroup(resp)) => {
                ("AddUserToGroup", resp.status)
            }
            Some(service_response::service_response::Operation::RemoveUserFromGroup(resp)) => {
                ("RemoveUserFromGroup", resp.status)
            }
            Some(service_response::service_response::Operation::ExitFromGroup(resp)) => {
                ("ExitFromGroup", resp.status)
            }
            Some(service_response::service_response::Operation::GetConversation(resp)) => {
                ("GetConversation", resp.status)
            }
            Some(service_response::service_response::Operation::ClearConversation(resp)) => {
                ("ClearConversation", resp.status)
            }
            Some(service_response::service_response::Operation::SendMessage(resp)) => {
                ("SendMessage", resp.status)
            }
            Some(service_response::service_response::Operation::RemoveMessage(resp)) => {
                ("RemoveMessage", resp.status)
            }
            Some(service_response::service_response::Operation::EditMessage(resp)) => {
                ("EditMessage", resp.status)
            }
            Some(service_response::service_response::Operation::ListConversation(resp)) => {
                ("ListConversation", resp.status)
            }
            Some(service_response::service_response::Operation::ReadMessage(resp)) => {
                ("ReadMessage", resp.status)
            }
            Some(service_response::service_response::Operation::GetMessage(resp)) => {
                ("GetMessage", resp.status)
            }
            Some(service_response::service_response::Operation::GetGroup(resp)) => {
                ("GetGroup", resp.status)
            }
            Some(service_response::service_response::Operation::Follow(resp)) => {
                ("Follow", resp.status)
            }
            Some(service_response::service_response::Operation::UnFollow(resp)) => {
                ("UnFollow", resp.status)
            }
            Some(service_response::service_response::Operation::ListFollowers(resp)) => {
                ("ListFollowers", resp.status)
            }
            Some(service_response::service_response::Operation::AddAttachment(resp)) => {
                ("AddAttachment", resp.status)
            }
            Some(service_response::service_response::Operation::RemoveAttachment(resp)) => {
                ("RemoveAttachement", resp.status)
            }
            Some(service_response::service_response::Operation::ListAttachments(resp)) => {
                ("ListAttachements", resp.status)
            }
            Some(service_response::service_response::Operation::ReadAttchment(resp)) => {
                ("ReadAttachment", resp.status)
            }

            _ => ("Unknown", -1),
        };
        let _ = write!(f, "Operation - {}  status - {}", operation, status);
        Ok(())
    }
}
