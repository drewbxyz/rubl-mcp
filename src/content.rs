use rmcp::model::Content;
use toon::encode as toon_encode;

pub trait ToContent {
    fn to_content(&self) -> Result<Content, serde_json::Error>;
}

impl<T: serde::Serialize> ToContent for T {
    fn to_content(&self) -> Result<Content, serde_json::Error> {
        let json = serde_json::to_value(self)?;
        Ok(Content::text(toon_encode(&json, None)))
    }
}
