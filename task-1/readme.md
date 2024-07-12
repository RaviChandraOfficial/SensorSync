# Rust CRUD Operations with Axum, SQLx, and PostgreSQL
This project demonstrates how to implement CRUD (Create, Read, Update, Delete) operations using the Axum web framework, SQLx async runtime, and PostgreSQL database in Rust.

## Prerequisites
### Before running the project, ensure you have the following prerequisites installed:
1. Rust Programming Language (https://www.rust-lang.org/tools/install)
2. Set up the PostgreSQL database:
Create a new PostgreSQL database.
Update the database connection details in the src/db.rs file.

## start the serve by using the command: cargo run
The server will start at http://localhost:3000.



## API Endpoints
The following CRUD operations are supported:

1. GET /items: Retrieve all items.
2. GET /items/{id}: Retrieve an item by ID.
3. POST /items: Create a new item.
4. PUT /items/{id}: Update an item by ID.
5. DELETE /items/{id}: Delete an item by ID.



## Contributing
Contributions are welcome! Please feel free to open an issue or submit a pull request for any improvements or bug fixes.



