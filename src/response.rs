#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUser {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyUser {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUser {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUser {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUser {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePassword {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPassword {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignIn {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignOut {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Follow {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnFollow {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFollowers {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTweet {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTweet {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTweets {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTweet {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTweet {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadProfilePicture {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProfilePicture {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfilePicture {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepostTweet {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyToTweet {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditReply {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveReply {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReply {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplies {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactToTweet {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoReactToTweet {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Operation", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28")]
    pub operation: ::core::option::Option<response::Operation>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="1")]
        AddUser(super::AddUser),
        #[prost(message, tag="2")]
        VerifyUser(super::VerifyUser),
        #[prost(message, tag="3")]
        RemoveUser(super::RemoveUser),
        #[prost(message, tag="4")]
        UpdateProfilePicture(super::UploadProfilePicture),
        #[prost(message, tag="5")]
        RemoveProfilePicture(super::RemoveProfilePicture),
        #[prost(message, tag="6")]
        GetProfilePicture(super::GetProfilePicture),
        #[prost(message, tag="7")]
        UpdateUser(super::UpdateUser),
        #[prost(message, tag="8")]
        GetUser(super::GetUser),
        #[prost(message, tag="9")]
        ChangePassword(super::ChangePassword),
        #[prost(message, tag="10")]
        ResetPassword(super::ResetPassword),
        #[prost(message, tag="11")]
        SignIn(super::SignIn),
        #[prost(message, tag="12")]
        SignOut(super::SignOut),
        #[prost(message, tag="13")]
        Follow(super::Follow),
        #[prost(message, tag="14")]
        UnFollow(super::UnFollow),
        #[prost(message, tag="15")]
        ListFollowers(super::ListFollowers),
        #[prost(message, tag="16")]
        AddTweet(super::AddTweet),
        #[prost(message, tag="17")]
        RemoveTweet(super::RemoveTweet),
        #[prost(message, tag="18")]
        ListTweets(super::ListTweets),
        #[prost(message, tag="19")]
        GetTweet(super::GetTweet),
        #[prost(message, tag="20")]
        UpdateTweet(super::UpdateTweet),
        #[prost(message, tag="21")]
        RepostTweet(super::RepostTweet),
        #[prost(message, tag="22")]
        ReplyToTweet(super::ReplyToTweet),
        #[prost(message, tag="23")]
        EditReply(super::EditReply),
        #[prost(message, tag="24")]
        RemoveReply(super::RemoveReply),
        #[prost(message, tag="25")]
        GetReply(super::GetReply),
        #[prost(message, tag="26")]
        ListReplies(super::ListReplies),
        #[prost(message, tag="27")]
        ReactToTweet(super::ReactToTweet),
        #[prost(message, tag="28")]
        UndoReactToTweet(super::UndoReactToTweet),
    }
}
