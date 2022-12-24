use axum::{extract::rejection::JsonRejection, http::StatusCode};

pub fn post_error_responder(err: JsonRejection) -> (StatusCode, String) {
    let response_text = match err {
        JsonRejection::JsonDataError(_err) => "Invalid data",
        JsonRejection::JsonSyntaxError(_err) => "Invalid json syntax",
        // handle other rejections from the `Json` extractor
        JsonRejection::MissingJsonContentType(_) => {
            "Missing `Content-Type: application/json` header"
        }
        JsonRejection::BytesRejection(_) => "Failed to buffer request body",
        // we must provide a catch-all case since `JsonRejection` is marked
        // `#[non_exhaustive]`
        _ => "Unknown error",
    };
    (StatusCode::BAD_REQUEST, response_text.to_string())
}
