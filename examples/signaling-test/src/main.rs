use std::collections::HashMap;

use tokio;
use warp::{self, Filter};

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "CineMotion Server");

    let session = warp::path!("sessions")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_sessions);

    let routes = root.or(session);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_sessions(
    body: HashMap<String, String>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    println!("body -> {:?}", body);
    Ok(warp::http::StatusCode::OK)
}
