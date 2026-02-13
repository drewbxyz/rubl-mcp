use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};

pub trait Endpoint {
    type Query: Serialize;
    type Response: DeserializeOwned;

    const METHOD: Method;

    fn path(&self) -> String;

    fn query(&self) -> &Self::Query;

    /// Some eBird endpoints default to CSV; return `Some("json")` to force JSON.
    fn format(&self) -> Option<&'static str> {
        None
    }
}
