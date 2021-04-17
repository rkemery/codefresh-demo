use httptest::{Server, Expectation, matchers::*, responders::*};
// Start a server running on a local ephemeral port.
let server = Server::run();
// Configure the server to expect a single GET /health request and respond
// with a 200 status code.
server.expect(
    Expectation::matching(request::method_path("GET", "/health"))
    .respond_with(status_code(200)),
);

// The server provides server.addr() that returns the address of the
// locally running server, or more conveniently provides a server.url() method
// that gives a fully formed http url to the provided path.
let url = server.url("/health");
let client = hyper::Client::new();
// Issue the GET /foo to the server.
let resp = client.get(url).await.unwrap();

// assert the response has a 200 status code.
assert!(resp.status().is_success());

// on Drop the server will assert all expectations have been met and will
// panic if not.