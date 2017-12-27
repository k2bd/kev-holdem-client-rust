extern crate hyper;

use hyper::{Method, Request};
use hyper::header::{ContentLength, ContentType};

fn main() {
    // Register to the game
    let reg_json = r#"
            {
                "Name" : "Kevin",
                "Address" : "localhost:8001"
            }
        "#;

    let uri = "localhost:8000/reg".parse()?;
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());

        res.body().concat2()
    });
}
