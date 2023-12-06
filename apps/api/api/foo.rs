use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "runtime": "rust",
                "path": "/api/foo",
            })
            .to_string()
            .into(),
        )?)
}
