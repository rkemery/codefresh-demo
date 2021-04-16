const request = require("supertest")("http://localhost:3000");
const expect = require("chai").expect;

describe("GET /", function () {
  it("returns status OK", async function () {
    const response = await request.get("/");

    expect(response.status).to.eql("Hello World!");
  });
});