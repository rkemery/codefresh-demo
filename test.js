request = require("request");
should = require("should");

describe('Applications API', function() {
  it('Checks existence of test application', function(done) {
    request.get('http://127.0.0.1:3000', function(err, response, body) {
      response.statusCode.should.equal(200);
      body.should.include("Hello World");
      done();
    })
  });
});