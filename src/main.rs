use axum::{routing::any, Router};

mod handlers {
    pub mod root;
    pub mod utils {
        pub mod invoke;
        pub mod request;
    }
}

use handlers::root;

#[tokio::main]
async fn main() {
    println!("Starting server at :3000");
    // build our application with a single route
    let app = Router::new().fallback(any(|| async { root::handle().await }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
