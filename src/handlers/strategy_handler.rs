package handlers;

use hyper::{Request, Response, Body};
use serde_json::Value;
use hyper::body::to_bytes;

pub struct Strategy {
    _activated: bool,
}

impl Strategy {
    pub fn set_activated(&mut self, activated: bool) {
        self._activated = activated;
    }
}

pub struct StrategyHandler {
    strategy: Strategy,
}

impl StrategyHandler {
    pub fn new() -> Self {
        StrategyHandler {
            strategy: Strategy { _activated: false },
        }
    }

    pub async fn set_activated_handler(&mut self, req: Request<Body>) -> Response<Body> {
        let body_bytes = to_bytes(req.into_body()).await;
        match body_bytes {
            Ok(bytes) => {
                let json: Result<Value, _> = serde_json::from_slice(&bytes);
                match json {
                    Ok(value) => {
                        if let Some(activated) = value.get("activated").and_then(Value::as_bool) {
                            self.strategy.set_activated(activated);
                            Response::new(Body::from("Activation status updated successfully"))
                        } else {
                            Response::builder()
                                .status(400)
                                .body(Body::from("Invalid input: expected a boolean value for 'activated'"))
                                .unwrap()
                        }
                    }
                    Err(_) => Response::builder()
                        .status(400)
                        .body(Body::from("Invalid JSON format"))
                        .unwrap(),
                }
            }
            Err(_) => Response::builder()
                .status(400)
                .body(Body::from("Failed to read request body"))
                .unwrap(),
        }
    }
}