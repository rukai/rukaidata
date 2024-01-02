use axum::Router;
use tower_http::services::ServeDir;

pub fn serve() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(run())
}

async fn run() {
    // build our application with a route
    let app = Router::new().nest_service("/", ServeDir::new("../root").precompressed_gzip());

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
