use dotenv;
use mpesa::{Environment, Mpesa};
use std::env;

#[test]
fn b2c_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .b2c("testapi496")
        .party_a("600496")
        .party_b("254708374149")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .amount(1000)
        .send();

    assert!(response.is_ok())
}
