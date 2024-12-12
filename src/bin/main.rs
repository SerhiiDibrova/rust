use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use warp::Filter;
use manual_order::ManualOrder;

#[tokio::main]
async fn main() {
    let orders = Arc::new(Mutex::new(Vec::new()));

    let orders_filter = {
        let orders = Arc::clone(&orders);
        warp::path("order")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |strategy: String| {
                let mut orders = orders.lock().unwrap();
                match ManualOrder::new(strategy) {
                    Ok(order) => {
                        orders.push(order);
                        warp::reply::json(&"Order created")
                    }
                    Err(_) => warp::reply::with_status("Invalid strategy", warp::http::StatusCode::BAD_REQUEST),
                }
            })
    };

    let addr: SocketAddr = "127.0.0.1:3030".parse().unwrap();
    warp::serve(orders_filter).run(addr).await;
}