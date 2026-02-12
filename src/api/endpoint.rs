use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};

pub trait Endpoint {
    type Query: Serialize;
    type Response: DeserializeOwned;

    const METHOD: Method;

    fn path(&self) -> &str;
    fn query(&self) -> &Self::Query;
}
