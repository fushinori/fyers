use fyers::Fyers;
use httpmock::MockServer;

pub struct TestContext {
    pub server: MockServer,
    pub fyers: Fyers,
}

pub fn setup() -> TestContext {
    let server = MockServer::start();
    let fyers = Fyers::with_base_urls(
        "TEST_CLIENT_ID",
        "TEST_ACCESS_TOKEN",
        server.base_url(),
        server.base_url(),
    );

    TestContext { server, fyers }
}
