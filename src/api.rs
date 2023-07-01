use crate::db::Database;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

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

#[get("/todos")]
async fn get_all_todos(db: web::Data<Database>) -> impl Responder {
    match db.get_tasks().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}

#[post("/todos")]
async fn create_todo() -> impl Responder {
    HttpResponse::Ok().body("todos")
}

#[get("/todos/{id}")]
async fn get_todo(path: web::Path<i64>, db: web::Data<Database>) -> impl Responder {
    let id: i64 = path.into_inner();

    match db.get_task_by_id(id).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}

#[put("/todos/{id}")]
async fn update_todo() -> impl Responder {
    HttpResponse::Ok().body("todos")
}

#[delete("/todos/{id}")]
async fn delete_todo(path: web::Path<i64>, db: web::Data<Database>) -> impl Responder {
    let id: i64 = path.into_inner();

    match db.delete_task(id).await {
        Ok(_) => HttpResponse::Ok().body("Successfully deleted with id: {id}"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
