extern crate postgres;

use postgres::{Connection};

#[macro_use]
use serde_derive::Serialize;
#[macro_use]
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id : i32,
    pub user_name: String
}


pub trait ById {
    fn user_of(id : i32, conn: Connection) -> Self;
}

impl ById for User {
    fn user_of(id: i32, conn: Connection) -> User {
        for row in &conn.query("SELECT id, username FROM users", &[]).unwrap() {
            let user = User {
                id: row.get(0),
                user_name: row.get(1)
            };

            return user;
        }

        return User {
            id: -1,
            user_name: "".to_string()
        };
    }
}


