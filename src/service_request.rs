#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisObject {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub password: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag="4")]
    pub email_otp: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUser {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheItem {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCacheItem {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCache {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToDb {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub password_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub password: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="4")]
    pub followers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="5")]
    pub profile_picture: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUser {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUser {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUser {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetProfilePicture {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub blob_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfilePicture {
    #[prost(bytes="vec", tag="1")]
    pub blob_name: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProfilePicture {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub blob_name: ::prost::alloc::vec::Vec<u8>,
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
    #[prost(int32, tag="7")]
    pub love_count: i32,
    #[prost(int32, tag="8")]
    pub dislike_count: i32,
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
    #[prost(uint64, tag="8")]
    pub created_at: u64,
    #[prost(string, repeated, tag="9")]
    pub hash_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="10")]
    pub user_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTweet {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(enumeration="TweetAdd", tag="2")]
    pub tweet_add: i32,
    #[prost(bytes="vec", tag="3")]
    pub tweet_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTweet {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTweets {
    #[prost(string, tag="1")]
    pub user_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTweet {
    #[prost(bytes="vec", tag="1")]
    pub tweet_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTweet {
    #[prost(enumeration="TweetRemove", tag="1")]
    pub tweet_remove: i32,
    #[prost(bytes="vec", tag="2")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddReply {
    #[prost(message, optional, tag="1")]
    pub reply: ::core::option::Option<Reply>,
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
pub struct UpdateReply {
    #[prost(bytes="vec", tag="1")]
    pub reply_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub hash_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub user_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes="vec", tag="6")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplies {
    #[prost(bytes="vec", tag="1")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReply {
    #[prost(bytes="vec", tag="1")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub reply_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TweetReact {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub tweet_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceRequest {
    #[prost(oneof="service_request::Operation", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22")]
    pub operation: ::core::option::Option<service_request::Operation>,
}
/// Nested message and enum types in `ServiceRequest`.
pub mod service_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="1")]
        AddUser(super::AddUser),
        #[prost(message, tag="2")]
        CacheItem(super::CacheItem),
        #[prost(message, tag="3")]
        GetCacheItem(super::GetCacheItem),
        #[prost(message, tag="4")]
        ClearCache(super::ClearCache),
        #[prost(message, tag="5")]
        AddToDb(super::AddToDb),
        #[prost(message, tag="6")]
        GetUser(super::GetUser),
        #[prost(message, tag="7")]
        RemoveUser(super::RemoveUser),
        #[prost(message, tag="8")]
        UpdateUser(super::UpdateUser),
        #[prost(message, tag="9")]
        SetProfilePicture(super::SetProfilePicture),
        #[prost(message, tag="10")]
        GetProfilePicture(super::GetProfilePicture),
        #[prost(message, tag="11")]
        RemoveProfilePicture(super::RemoveProfilePicture),
        #[prost(message, tag="12")]
        AddTweet(super::AddTweet),
        #[prost(message, tag="13")]
        GetTweet(super::GetTweet),
        #[prost(message, tag="14")]
        RemoveTweet(super::RemoveTweet),
        #[prost(message, tag="15")]
        ListTweets(super::ListTweets),
        #[prost(message, tag="16")]
        UpdateTweet(super::UpdateTweet),
        #[prost(message, tag="17")]
        TweetReact(super::TweetReact),
        #[prost(message, tag="18")]
        AddReply(super::AddReply),
        #[prost(message, tag="19")]
        UpdateReply(super::UpdateReply),
        #[prost(message, tag="20")]
        RemoveReply(super::RemoveReply),
        #[prost(message, tag="21")]
        GetReply(super::GetReply),
        #[prost(message, tag="22")]
        ListReplies(super::ListReplies),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TweetRemove {
    OwnTweet = 0,
    RepostedTweet = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TweetAdd {
    Add = 0,
    Repost = 1,
}
