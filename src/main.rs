#![feature(never_type)]
use lambda_runtime::{self, handler_fn, Context, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(func)).await
}

async fn func(value: Value, _: Context) -> Result<impl serde::Serialize, !> {
    if let Some(path) = value
        .get("pathParameters")
        .map(|pps| pps.get("urlParam"))
        .flatten()
        .map(|p| {
            if let Value::String(s) = p {
                Some(s.as_str())
            } else {
                None
            }
        })
        .flatten()
    {
        match path {
            "test" => {
                return Ok(json!({
                    "statusCode": u16::from(http::StatusCode::OK),
                    "body": "Dette er en test",
                    "isBase64Encoded": false
                }))
            }
            "dispensation" | "d" => {
                return Ok(json!({
                    "statusCode": u16::from(http::StatusCode::FOUND),
                    "headers": {
                        "location": "https://game.academy.beer/login"
                    },
                    "body": "Redirecting...",
                    "isBase64Encoded": false
                }))
            }
            _ => (),
        }
    }
    Ok(json!({
        "statusCode": u16::from(http::StatusCode::FOUND),
        "headers": {
            "location": "https://brightspace.au.dk"
        },
        "body": "Redirecting...",
        "isBase64Encoded": false
    }))
}
