#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUser {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheItem {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCacheItem {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag="3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCache {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToDb {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
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
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(message, optional, tag="2")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUser {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUser {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetProfilePicture {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfilePicture {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(bytes="vec", tag="2")]
    pub blob: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProfilePicture {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tweet {
    #[prost(bytes="vec", tag="1")]
    pub tweet_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub created_at: ::prost::alloc::string::String,
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
pub struct AddTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(bytes="vec", tag="2")]
    pub tweet_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTweets {
    #[prost(bytes="vec", repeated, tag="1")]
    pub tweet_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTweet {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceResponse {
    #[prost(oneof="service_response::Operation", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16")]
    pub operation: ::core::option::Option<service_response::Operation>,
}
/// Nested message and enum types in `ServiceResponse`.
pub mod service_response {
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
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Success = 0,
    BackendError = 1,
}
