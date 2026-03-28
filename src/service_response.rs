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
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCacheItem {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCache {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToDb {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
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
pub struct ServiceResponse {
    #[prost(oneof="service_response::Operation", tags="1, 2, 3, 4, 5, 6, 7")]
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
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Success = 0,
    BackendError = 1,
}
