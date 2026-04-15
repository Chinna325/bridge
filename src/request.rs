#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUser {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyUser {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email_otp: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUser {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUser {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUser {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePassword {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub old_password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPassword {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignIn {
    #[prost(string, optional, tag="1")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignOut {
    #[prost(string, optional, tag="1")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
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
    #[prost(message, optional, tag="1")]
    pub tweet: ::core::option::Option<Tweet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTweet {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTweets {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTweet {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTweet {
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub hash_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub user_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="4")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadProfilePicture {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProfilePicture {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfilePicture {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepostTweet {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyToTweet {
    #[prost(message, optional, tag="1")]
    pub reply: ::core::option::Option<Reply>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reply {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub text: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub reply_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="6")]
    pub likes: i32,
    #[prost(int32, tag="7")]
    pub dislikes: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditReply {
    #[prost(bytes="vec", tag="1")]
    pub reply_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="3")]
    pub hash_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub user_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="5")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveReply {
    #[prost(bytes="vec", tag="1")]
    pub reply_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReply {
    #[prost(bytes="vec", tag="1")]
    pub reply_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplies {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactToTweet {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="TweetReact", tag="2")]
    pub tweet_react: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoReactToTweet {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="UndoTweetReact", tag="2")]
    pub tweet_react: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tweet {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub created_at: u64,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    /// PublicMetrics publicMetrics=5;
    #[prost(string, repeated, tag="6")]
    pub hashtags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="7")]
    pub user_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOneToOneChat {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroup {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub users: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGroup {
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroup {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroups {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToGroup {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserFromGroup {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExitFromGroup {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChat {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearChat {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessage {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub content: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMessage {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub message_id: u64,
    #[prost(enumeration="MessageRemove", tag="3")]
    pub message_remove: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditMessage {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub message_id: u64,
    #[prost(string, tag="3")]
    pub content: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChat {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMessage {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub message_id: u64,
}
///create group
///delete group
/// clear chat
///get chat
///list chats
///update chat
///add usertochat
///remove user to chat
///send message
///edit message
///delete message
///list messages
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::Operation", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43")]
    pub operation: ::core::option::Option<request::Operation>,
}
/// Nested message and enum types in `Request`.
pub mod request {
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
        #[prost(message, tag="29")]
        CreateOneToOneChat(super::CreateOneToOneChat),
        #[prost(message, tag="30")]
        CreateGroup(super::CreateGroup),
        #[prost(message, tag="31")]
        RemoveGroup(super::RemoveGroup),
        #[prost(message, tag="32")]
        UpdateGroup(super::UpdateGroup),
        #[prost(message, tag="33")]
        ListGroups(super::ListGroups),
        #[prost(message, tag="34")]
        AddUserToGroup(super::AddUserToGroup),
        #[prost(message, tag="35")]
        RemoveUserFromGroup(super::RemoveUserFromGroup),
        #[prost(message, tag="36")]
        ExitFromGroup(super::ExitFromGroup),
        #[prost(message, tag="37")]
        GetChat(super::GetChat),
        #[prost(message, tag="38")]
        ClearChat(super::ClearChat),
        #[prost(message, tag="39")]
        SendMessage(super::SendMessage),
        #[prost(message, tag="40")]
        RemoveMessage(super::RemoveMessage),
        #[prost(message, tag="41")]
        EditMessage(super::EditMessage),
        #[prost(message, tag="42")]
        ListChat(super::ListChat),
        #[prost(message, tag="43")]
        ReadMessage(super::ReadMessage),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TweetReact {
    Like = 0,
    Love = 1,
    DisLike = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UndoTweetReact {
    UndoLike = 0,
    UndoDisLike = 1,
    UndoLove = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageRemove {
    RemoveForme = 0,
    RemoveForAll = 1,
}
