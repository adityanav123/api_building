use crate::db;

use super::models::{Book, NewBook};
use db::Conn as DbConn;
use rocket::{catch, delete, get, post, put, serde::json::Json};
use serde_json::{json, Value};

// Fetch all books in the database.
#[get("/books", format = "application/json")]
pub fn index(mut conn: DbConn) -> Json<Value> {
    let books = Book::all(&mut conn);
    Json(json!({
        "status": 200,
        "result": books,
    }))
}

// Add a new book to the database.
#[post("/books", format = "application/jauthorson", data = "<new_book>")]
pub fn new(mut conn: DbConn, new_book: Json<NewBook>) -> Json<Value> {
    Json(json!({
        "status": Book::insert(new_book.into_inner(), &mut conn),
        "result": Book::all(&mut conn).first()
    }))
}

// Fetch a single book by its ID.
#[get("/books/<id>", format = "application/json")]
pub fn show(mut conn: DbConn, id: i32) -> Json<Value> {
    let result = Book::show(id, &mut conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status" : status,
        "result": result.first()
    }))
}

// Update a book's details by its ID.
#[put("/books/<id>", format = "application/json", data = "<book>")]
pub fn update(mut conn: DbConn, id: i32, book: Json<NewBook>) -> Json<Value> {
    let status = if Book::update_by_id(id, &mut conn, book.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status" : status,
        "result": null
    }))
}

// Delete a book by its ID.
#[delete("/books/<id>")]
pub fn delete(id: i32, mut conn: DbConn) -> Json<Value> {
    let status = if Book::delete_by_id(id, &mut conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null
    }))
}

// Fetch all books by a specific author.
#[get("/book/author/<author_name>", format = "application/json")]
pub fn author_books(author_name: String, mut conn: DbConn) -> Json<Value> {
    let result = Book::all_by_author(author_name.clone(), &mut conn);

    Json(json!({
        "status": 200,
        "result": result// Returns all books by the author.
    }))
}

// Custom error handler for 404 Not Found.
#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found!"
    }))
}
