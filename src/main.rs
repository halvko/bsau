#![feature(never_type)]
use lambda_runtime::{self, service_fn, Error, LambdaEvent};
use phf::phf_map;
use serde_json::{json, Value};

const RC: &str = "https://www.facebook.com/groups/regnecentralen.au";
const ACADEMY: &str = "https://game.academy.beer";
const AMC: &str = "https://brightspace.au.dk/d2l/home/54990";

static COURSES: phf::Map<&'static str, &'static str> = phf_map! {
    "amc" => AMC,
    "hovedkontor" => RC, "regnecentralen" => RC, "rc" => RC,
    "d" => ACADEMY, "dispensation" => ACADEMY,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, !> {
    let (event, _cxt) = event.into_parts();
    let path = event["pathParameters"]
        .as_object()
        .and_then(|m| m.get("urlParam"))
        .and_then(Value::as_str)
        .unwrap_or("");
    match path {
        "test" | "oversigt" => Ok(json!({
            "statusCode": u16::from(http::StatusCode::OK),
            "headers": {
                "content-type": "text/html"
            },
            "body": include_str!("oversigt.html"),
            "isBase64Encoded": false
        })),
        s if s.starts_with("toast") => {
            if rand::random::<u8>() <= 25 {
                return Ok(json!({
                    "statusCode": u16::from(http::StatusCode::OK),
                    "headers": {
                        "content-type": "text/html"
                    },
                    "body": include_str!("bamboozeld.html"),
                    "isBase64Encoded": false
                }));
            }
            Ok(json!({
                "statusCode": u16::from(http::StatusCode::FOUND),
                "headers": {
                    "location": "https://bamboozle.academy"
                },
                "body": "Redirecting...",
                "isBase64Encoded": false
            }))
        }
        rest => {
            let &url = COURSES.get(rest).unwrap_or(&"https://brightspace.au.dk");
            Ok(json!({
                "statusCode": u16::from(http::StatusCode::FOUND),
                "headers": {
                    "location": url
                },
                "body": "Redirecting...",
                "isBase64Encoded": false
            }))
        }
    }
}
