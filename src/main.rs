use axum::{routing::get, Router, response::Json};
use serde::Serialize;
use mysql::*;
#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[tokio::main]
async fn main() {
    // Create the Axum router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));
    let url = "mysql://root:root@localhost:3306/task_manager";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let url = "mysql://root:root@localhost:3306/task_manager";
    let pool = Pool::new(url).unwrap();
    //creating a connection
    let mut conn = pool.get_conn().unwrap();

    // let mut username = String::new();
    // println!("Enter your employee id :");
    // std::io::stdin().read_line(&mut username).unwrap();
    // let empid: i64 = username.trim().parse().expect("username");
    // println!("Employee ID:{}", empid);

    // let mut pswrd = String::new();
    // println!("Enter your password :");
    // std::io::stdin().read_line(&mut pswrd).unwrap();

    // Bind the server to 0.0.0.0:8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("Server running on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}

// Handler for the root route
async fn root() -> &'static str {
    "Welcome to the Rust API!"
}

// Handler for the /health route
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}