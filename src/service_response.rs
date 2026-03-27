#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUser {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheItem {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="Status", tag="2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToDb {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceResponse {
    #[prost(oneof="service_response::Operation", tags="1, 2, 3")]
    pub operation: ::core::option::Option<service_response::Operation>,
}
/// Nested message and enum types in `ServiceResponse`.
pub mod service_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="1")]
        AddUser(super::AddUser),
        #[prost(message, tag="2")]
        AcheItem(super::CacheItem),
        #[prost(message, tag="3")]
        AddToDb(super::AddToDb),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Success = 0,
    BackendError = 1,
}
