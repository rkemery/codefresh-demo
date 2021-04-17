extern crate hyper;

use hyper::*;

#[test]
fn get_backend() {
    let client = Client::new();

    let res = client.get("http://localhost:8000/health").send().unwrap();

    assert_eq!(res.status, hyper::Ok);
}