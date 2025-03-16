use axum::{middleware::map_response, response::Response, routing, Router};
use tower_http::services::ServeDir;

pub fn serve() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(run())
}

async fn run() {
    // build our application with a route
    let app = Router::new().nest_service(
        "/",
        routing::get_service(ServeDir::new("../root"))
            .layer(map_response(insert_compression_header)),
    );

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// All files are compressed with gzip, so we need to overwrite content-encoding to let the client know.
// We cant use `ServeDir::precompressed_gzip` since name on disk needs to match name served for hosting on AWS S3 to work.
async fn insert_compression_header<B>(mut response: Response<B>) -> Response<B> {
    response
        .headers_mut()
        .insert("content-encoding", "gzip".parse().unwrap());
    response
}
