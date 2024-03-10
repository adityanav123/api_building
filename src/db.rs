// decouple our db

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::{request, Request, State};
use std::ops::{Deref, DerefMut};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool failure")
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Conn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<&State<Pool>>().await;
        match pool {
            Outcome::Success(pool) => match pool.get() {
                Ok(conn) => Outcome::Success(Conn(conn)),
                Err(_) => Outcome::Error((Status::ServiceUnavailable, ())),
            },
            _ => Outcome::Error((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Conn {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
