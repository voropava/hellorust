#[macro_use]

extern crate postgres;
extern crate hyper;

extern crate serde_json;
extern crate serde;

#[macro_use]
use serde_derive::{Deserialize, Serialize};

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use postgres::{Connection, TlsMode};
use serde_json::json;

mod mydao;


fn main() {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 8080).into();

    let user = mydao::User {
        id : 1,
        user_name : "a".to_owned()
    };

    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(move |_: Request<Body>| {
            let conn = Connection::connect(
                "postgres://postgres:postgres@localhost:5432/postgres",
                TlsMode::None
            ).unwrap();

            let user : mydao::User = mydao::ById::user_of(1, conn);

            return Response::new(Body::from(serde_json::to_string(&user).unwrap()));
        })
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    eprint!("Starting server...");

    hyper::rt::run(server);
}


fn vector() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(10);
    v.push(12);


    return v;
}

#[test]
fn testVector() {
    let v = vector();

    assert!(v[0] == 1);
}
