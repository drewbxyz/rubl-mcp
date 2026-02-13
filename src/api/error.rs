use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    /// HTTP error with response body (often contains helpful API error messages).
    #[error("eBird API returned {status}: {body}")]
    HttpError {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Network error: {0}")]
    Network(reqwest::Error),

    #[error("Failed to deserialize API response: {0}")]
    Deserialization(reqwest::Error),

    #[error("Failed to serialize request parameters: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    mod display {
        use super::*;

        #[test]
        fn http_error_includes_status_and_body() {
            let error = ApiError::HttpError {
                status: StatusCode::BAD_REQUEST,
                body: "Invalid region code".into(),
            };
            let display = format!("{}", error);
            assert!(display.contains("400"));
            assert!(display.contains("Invalid region code"));
        }

        #[test]
        fn http_error_with_404() {
            let error = ApiError::HttpError {
                status: StatusCode::NOT_FOUND,
                body: "Region not found".into(),
            };
            let display = format!("{}", error);
            assert!(display.contains("404"));
            assert!(display.contains("Region not found"));
        }

        #[tokio::test]
        async fn network_error_includes_message() {
            // Create a network error by attempting to connect to an invalid domain
            let reqwest_error = reqwest::Client::new()
                .get("http://invalid-domain-that-does-not-exist-12345.com")
                .send()
                .await
                .unwrap_err();

            let error = ApiError::Network(reqwest_error);
            let display = format!("{}", error);
            assert!(display.contains("Network error"));
        }

        #[test]
        fn serialization_error_includes_message() {
            // Create a serialization error by trying to serialize invalid JSON
            let json_error = serde_json::from_str::<serde_json::Value>("{invalid}").unwrap_err();
            let error = ApiError::Serialization(json_error);
            let display = format!("{}", error);
            assert!(display.contains("Failed to serialize request parameters"));
        }

        #[test]
        fn http_error_with_500() {
            let error = ApiError::HttpError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: "Server error occurred".into(),
            };
            let display = format!("{}", error);
            assert!(display.contains("500"));
            assert!(display.contains("Server error occurred"));
        }
    }
}
