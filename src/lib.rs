pub mod better;
pub use better::*;

pub mod client;
pub use client::*;

pub mod jwt;
pub mod response;

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::client::{EnvConfig, NPClient};
    use crate::better::currencies::Currency;
    use crate::client::PaymentOpts;

    fn client() -> NPClient {
        EnvConfig::client()
    }

    fn sandbox_client() -> NPClient {
        EnvConfig::sandbox_client()
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
        c.get_min_payment_amount(Currency::ETH, Currency::BTC)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_estimate_price() {
        let mut c = client();
        // panics if not error
        c.price()
            .amount(2000.0)
            .from(Currency::BTC)
            .to(Currency::ETH)
            .get()
            .await;
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn authentication() {
        let mut c = client();

        let config = EnvConfig::parse();
        c.set_auth(config.email, config.password);

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

        let config = EnvConfig::parse();
        c.set_auth(config.email, config.password);

        // panics if not error
        c.authenticate().await.unwrap();

        c.get_payment(1).await.unwrap();
    }
}
