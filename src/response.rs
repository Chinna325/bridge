#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUser {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyUser {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUser {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUser {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUser {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePassword {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPassword {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignIn {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignOut {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Follow {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnFollow {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFollowers {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="3")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTweets {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub tweets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub tweet: ::core::option::Option<Tweet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadProfilePicture {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProfilePicture {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfilePicture {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepostTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyToTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="3")]
    pub reply_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditReply {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveReply {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReply {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub reply: ::core::option::Option<Reply>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplies {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub replies: ::prost::alloc::vec::Vec<Reply>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactToTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoReactToTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub create_at: u64,
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
    #[prost(message, optional, tag="5")]
    pub public_metrics: ::core::option::Option<PublicMetrics>,
    #[prost(string, repeated, tag="6")]
    pub hashtags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="7")]
    pub user_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicMetrics {
    #[prost(int32, tag="1")]
    pub retweet_count: i32,
    #[prost(int32, tag="2")]
    pub reply_count: i32,
    #[prost(int32, tag="3")]
    pub like_count: i32,
    #[prost(int32, tag="4")]
    pub quote_count: i32,
    #[prost(int32, tag="5")]
    pub bookmark_count: i32,
    #[prost(int32, tag="6")]
    pub impression_count: i32,
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
pub struct Message {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub content: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub created_at: u64,
    #[prost(uint64, tag="4")]
    pub message_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOneToOneChat {
    #[prost(bytes="vec", tag="1")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="Status", tag="2")]
    pub status: i32,
    #[prost(string, optional, tag="3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroup {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="3")]
    pub chat_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGroup {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroup {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroups {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToGroup {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserFromGroup {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExitFromGroup {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChat {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearChat {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessage {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMessage {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditMessage {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChat {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    #[prost(bytes="vec", tag="1")]
    pub group_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="2")]
    pub users: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub created_by: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub created_at: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMessage {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Operation", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43")]
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
pub enum Status {
    Success = 0,
    BackendError = 1,
}
