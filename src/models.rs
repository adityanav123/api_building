use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::books;
use crate::schema::books::dsl::books as all_books;

#[derive(Queryable, Debug, Serialize, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Book {
    pub fn show(id: i32, conn: &mut PgConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error Loading Book")
    }

    pub fn all(conn: &mut PgConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("Error loading the Book")
    }

    pub fn update_by_id(id: i32, conn: &mut PgConnection, book: NewBook) -> bool {
        use crate::schema::books::dsl::{author as a, published as p, title as t};
        let NewBook {
            title,
            author,
            published,
        } = book;

        diesel::update(all_books.find(id))
            .set((a.eq(author), p.eq(published), t.eq(title)))
            .get_result::<Book>(conn)
            .is_ok()
    }

    pub fn insert(book: NewBook, conn: &mut PgConnection) -> bool {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &mut PgConnection) -> bool {
        if Book::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_books.find(id)).execute(conn).is_ok()
    }

    pub fn all_by_author(author: String, conn: &mut PgConnection) -> Vec<Book> {
        all_books
            .filter(books::author.eq(author))
            .load::<Book>(conn)
            .expect("Error loading Books By Author")
    }
}
