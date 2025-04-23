use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use log::error;
use uuid::{Uuid, Version};

/// This is a custom extractor for Axum that extracts the request id, via the `X-Request-Id` header.
/// If the `X-Request-Id` header is present and it's a valid UUID v7, it returns it.
/// If the `X-Request-Id` header is present and it's an invalid UUID v7 (either not an UUID or an UUID v7), it returns a 400 Bad Request error with a specific message.
/// If the `X-Request-Id` header is not present, it defaults to a newly generated UUID v7.
///
/// # Links
///
/// https://docs.rs/axum/latest/axum/index.html
/// https://docs.rs/axum/latest/axum/extract/index.html#defining-custom-extractors
/// https://docs.rs/uuid/latest/uuid/index.html
///
/// # Author
///
/// François GRUCHALA <francois@nebeto.xyz>
///
/// # Examples
///
/// ```rust
/// use axum::{routing::get, Router};
/// use request_id_middleware::ExtractRequestId;
///
/// async fn handler(ExtractRequestId(request_id): ExtractRequestId) {
///     println!("Request Id: {:?}", request_id);
/// }
///
/// let app = Router::<()>::new().route("/foo", get(handler));
/// ```
#[derive(Debug, Clone)]
pub struct ExtractRequestId(pub String);

const HEADER_X_REQUEST_ID: &str = "X-Request-Id";

impl<S> FromRequestParts<S> for ExtractRequestId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let request_id = parts.headers.get(HEADER_X_REQUEST_ID);

        match request_id {
            Some(request_id) => {
                let request_id = request_id.to_str().unwrap().trim().to_lowercase();
                let parsed_request_id = Uuid::try_parse(request_id.as_str());

                if parsed_request_id.is_err() {
                    error!(
                        "[{}] Failed to parse UUID due to : {:?}",
                        HEADER_X_REQUEST_ID,
                        parsed_request_id.err().unwrap()
                    );

                    return Err((
                        StatusCode::BAD_REQUEST,
                        format!("Invalid {} : Not a valid UUID", HEADER_X_REQUEST_ID),
                    ));
                }

                let request_id_version = parsed_request_id.unwrap().get_version().unwrap();

                if request_id_version != Version::SortRand {
                    error!(
                        "[{}] Failed to validate UUID due to : Version is {:?}",
                        HEADER_X_REQUEST_ID, request_id_version
                    );

                    return Err((
                        StatusCode::BAD_REQUEST,
                        format!("Invalid {} : Not an UUID v7", HEADER_X_REQUEST_ID),
                    ));
                }

                Ok(ExtractRequestId(request_id))
            }
            None => Ok(ExtractRequestId(Uuid::now_v7().to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ExtractRequestId, HEADER_X_REQUEST_ID};
    use axum::{
        body::Body,
        extract::FromRequestParts,
        http::{Request, StatusCode},
    };

    #[tokio::test]
    async fn test_lib_extract_request_id_with_header_ok_one() {
        let request = Request::builder()
            .header("x-request-id", "01965864-f8ab-7eb8-912a-a2c999ab110e")
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let request_id = ExtractRequestId::from_request_parts(&mut parts.0, &()).await;

        match request_id {
            Ok(request_id) => assert_eq!(request_id.0, "01965864-f8ab-7eb8-912a-a2c999ab110e"),
            Err(err) => assert!(false, "Expected a valid request id : {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_request_id_with_header_ok_two() {
        let request = Request::builder()
            .header("X-Request-Id", " 01965864-f8ab-7Eb8-912a-a2c999ab110e ")
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let request_id = ExtractRequestId::from_request_parts(&mut parts.0, &()).await;

        match request_id {
            Ok(request_id) => assert_eq!(request_id.0, "01965864-f8ab-7eb8-912a-a2c999ab110e"),
            Err(err) => assert!(false, "Expected a valid request id : {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_request_id_with_header_ko_not_uuid() {
        let request = Request::builder()
            .header("X-Request-ID", "this-is-not-a-uuid")
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let request_id = ExtractRequestId::from_request_parts(&mut parts.0, &()).await;

        match request_id {
            Ok(_) => assert!(false, "Expected an error"),
            Err(err) => assert_eq!(
                err,
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid {} : Not a valid UUID", HEADER_X_REQUEST_ID)
                )
            ),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_request_id_with_header_ko_not_uuid_v7() {
        let request = Request::builder()
            .header("x-Request-ID", "6edaba95-4f5b-4547-be3f-85210d3ff8bf")
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let request_id = ExtractRequestId::from_request_parts(&mut parts.0, &()).await;

        match request_id {
            Ok(_) => assert!(false, "Expected an error"),
            Err(err) => assert_eq!(
                err,
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid {} : Not an UUID v7", HEADER_X_REQUEST_ID)
                )
            ),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_request_id_without_header() {
        let request = Request::builder().body(Body::empty()).unwrap();

        let mut parts = request.into_parts();

        let request_id = ExtractRequestId::from_request_parts(&mut parts.0, &()).await;

        match request_id {
            Ok(_) => assert!(true),
            Err(err) => assert!(false, "Expected a valid request id : {:?}", err),
        }
    }
}
