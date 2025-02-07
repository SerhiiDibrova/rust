use crate::global_container;
use crate::strategy::Strategy;
use serde_json::json;
use warp::http::StatusCode;
use warp::Reply;

pub struct Response {
    pub status: StatusCode,
    pub body: String,
}

pub async fn process_request(request: Request) -> Result<Response, warp::Rejection> {
    match request.request_type {
        RequestType::LOGIN => {
            handle_login(request).await?;
            Ok(Response {
                status: StatusCode::OK,
                body: "Login successful".to_string(),
            })
        }
        RequestType::ORDER => {
            match request.order_type {
                OrderType::NEW => {
                    handle_new_order(request).await?;
                    Ok(Response {
                        status: StatusCode::CREATED,
                        body: "Order created".to_string(),
                    })
                }
                OrderType::MODIFY => {
                    handle_modify_order(request).await?;
                    Ok(Response {
                        status: StatusCode::OK,
                        body: "Order modified".to_string(),
                    })
                }
                OrderType::DELETE => {
                    handle_delete_order(request).await?;
                    Ok(Response {
                        status: StatusCode::OK,
                        body: "Order deleted".to_string(),
                    })
                }
            }
        }
        RequestType::SUBSCRIBE => {
            subscribe_to_strategy(request).await?;
            Ok(Response {
                status: StatusCode::OK,
                body: "Subscribed successfully".to_string(),
            })
        }
    }
}

pub async fn retrieve_strategy_ids() -> Result<impl Reply, warp::Rejection> {
    let container = global_container::get_global_strategy_container();
    let strategies = container.lock().unwrap();
    let mut strategy_id_list = Vec::new();
    for strategy in strategies.values() {
        let strategy_id = strategy.get_id();
        if !strategy_id_list.contains(&strategy_id.to_string()) {
            strategy_id_list.push(strategy_id.to_string());
        }
    }
    Ok(warp::reply::json(&strategy_id_list))
}