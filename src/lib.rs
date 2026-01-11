pub mod better;

pub mod client;
pub mod jwt;
pub mod response;

#[cfg(test)]
mod test {
    use std::env::var;
    use tracing_test::traced_test;

    use crate::better::currencies::Currency;
    use crate::client::PaymentOpts;

    use super::client::NPClient;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct Config {
        api_key: String,
        sandbox_api_key: String,
        email: String,
        password: String,
    }

    fn parse_config() -> Config {
        dotenvy::dotenv().unwrap();
        return Config {
            api_key: var("NOWPAYMENTS_API_KEY").unwrap(),
            sandbox_api_key: var("NOWPAYMENTS_SANDBOX_API_KEY").unwrap(),

            email: var("NOWPAYMENTS_EMAIL").unwrap_or("null".to_owned()),
            password: var("NOWPAYMENTS_PASSWORD").unwrap_or("null".to_owned()),
        };
    }

    fn client() -> NPClient {
        let config = parse_config();
        NPClient::new(config.api_key.as_str())
    }

    fn sandbox_client() -> NPClient {
        let config = parse_config();
        NPClient::new_sandbox(config.sandbox_api_key.as_str())
    }

    #[test]
    fn verify_client() {
        client();
    }

    #[test]
    fn verify_sandbox_client() {
        sandbox_client();
    }

    #[tokio::test]
    async fn get_status() {
        let mut c = client();

        let status = c.status().await.unwrap();

        assert_eq!(status.message, "OK".to_string())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_currencies() {
        let mut c = client();

        // panics if not error
        c.get_currencies().await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn get_full_currencies() {
        let mut c = client();

        // panics if not error
        c.get_full_currencies().await.unwrap();
    }

    #[tokio::test]
    async fn get_checked_currencies() {
        let mut c = client();

        // panics if not error
        c.get_checked_currencies().await.unwrap();
    }

    #[tokio::test]
    async fn get_min_payment_amount() {
        let mut c = client();

        // panics if not error
        c.get_min_payment_amount("eth", "btc").await.unwrap();
    }

    #[tokio::test]
    async fn get_estimate_price() {
        let mut c = client();

        // panics if not error
        c.get_estimated_price(2000, "btc", "eth").await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn authentication() {
        let mut c = client();

        let conf = parse_config();
        c.set_auth(conf.email, conf.password);

        // panics if not error
        c.authenticate().await.unwrap();

        // c.get_payout_list().await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn create_payment() {
        let payment = PaymentOpts::builder()
            .price_amount(100.0)
            .price_currency(Currency::USD)
            .pay_currency(Currency::XMR)
            .order_id("my_order_0")
            .order_description("my test order")
            .ipn_callback_url("https://test.com/")
            .build();

        let mut c = client();
        // let mut c = sandbox_client();

        c.create_payment(payment).await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn get_payment() {
        let ipn_callback = "https://test.com/";
        let payment = PaymentOpts::builder()
            .price_amount(100.0)
            .price_currency(Currency::USD)
            .pay_currency(Currency::XMR)
            .order_id("my_order_0")
            .order_description("my test order")
            .ipn_callback_url("https://test.com/")
            .build();

        let mut c = client();

        let conf = parse_config();
        c.set_auth(conf.email, conf.password);

        // panics if not error
        c.authenticate().await.unwrap();

        c.get_payment(1).await.unwrap();
    }
}
