use axum::{
    routing::{delete, get, post, put}, Router
};

use my_rest_api::handler;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use std::time::Duration;

//asynchronous main
#[tokio::main]
async fn main() {
    let db_connection_str= "postgres://people:123@localhost".to_string();
    // Initialize a connection pool to the PostgreSQL database with specific configurations.
    let pool = PgPoolOptions::new()
        .connect(&db_connection_str)                // Connect to the database using the connection string.
        .await         // Asynchronously wait for the connection to be established.
        .expect("can't connect to database");       // Panic if the connection cannot be established.

    println!("Connected to url:");

    // Setup the web server routes and associate them with their respective handler functions.
    let app = Router::new()
    .route("/get/user", get(handler::get_data))          // Route for fetching all users.
    .route("/get_id/user", get(handler::get_id_data))    // Route for fetching a user by ID.
    .route("/post/user", post(handler::post_data))       // Route for creating a new user.
    .route("/put/user", put(handler::put_data))          // Route for updating an existing user.
    .route("/delete/user", delete(handler::delete_data)) // Route for deleting a user.
    .with_state(pool);                                            // Attach the database connection pool to the application state.

    // Prepare a TCP listener on port 3000 of all network interfaces.
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

     // Launch the Axum web server to handle incoming HTTP requests.
    axum::serve(listener, app).await.unwrap();
}

