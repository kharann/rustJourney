// package name for the buffer will be used later

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Book {
    #[prost(string, tag = "1")]
    pub title: std::string::String,
    #[prost(string, tag = "2")]
    pub author: std::string::String,
}
pub mod book {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Genre {
        Fantasy = 0,
        Crime = 1,
    }
}
