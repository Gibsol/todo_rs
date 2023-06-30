use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    let welcoming_message = r#"
        Welcome to the Rust API!
        
        This API provides endpoints to manage your tasks and todos.
        You can create, retrieve, update, and delete todos using the provided endpoints.
        
        API Endpoints:
        - GET /todos: Retrieve all todos
        - POST /todos: Create a new todo
        - GET /todos/{id}: Retrieve a specific todo
        - PUT /todos/{id}: Update a todo
        - DELETE /todos/{id}: Delete a todo
        
        Please refer to the API documentation for detailed information on request/response structures and usage.
    "#;

    HttpResponse::Ok().body(welcoming_message)
}
