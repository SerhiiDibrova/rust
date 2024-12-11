package main

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use serde_json::json;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use strategy::Strategy;
use log::{info, LevelFilter};
use simplelog::{Config, SimpleLogger};

struct AppState {
    logger: Arc<Mutex<Logger>>,
}

async fn handle_activation_status(_req: Request<Body>, state: Arc<AppState>) -> Result<Response<Body>, hyper::Error> {
    let strategy = Strategy::new();
    let activated_status = strategy.activated();
    let response = json!({ "activated": activated_status });
    Ok(Response::new(Body::from(response.to_string())))
}

async fn handle_order_response(_req: Request<Body>, _state: Arc<AppState>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("Order response")))
}

async fn handle_connection_events(_req: Request<Body>, _state: Arc<AppState>) -> Result<Response<Body>, hyper::Error> {
    Adaptor::OnConnection();
    Ok(Response::new(Body::from("Connection events")))
}

async fn handle_update_arthur(_req: Request<Body>, _state: Arc<AppState>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("Update Arthur")))
}

fn main() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
    let logger = Arc::new(Mutex::new(Logger::new()));
    let state = Arc::new(AppState { logger });

    let make_svc = make_service_fn(|_| {
        let state = state.clone();
        async { Ok::<_, hyper::Error>(service_fn(move |req| {
            let state = state.clone();
            async move {
                match req.uri().path() {
                    "/strategy/activated" => handle_activation_status(req, state).await,
                    "/api/order-response" => handle_order_response(req, state).await,
                    "/connection/events" => handle_connection_events(req, state).await,
                    "/update_arthur" => handle_update_arthur(req, state).await,
                    _ => Ok(Response::new(Body::from("Not Found"))),
                }
            }
        })) 
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    let rt = Runtime::new().unwrap();
    rt.block_on(server).unwrap();
}