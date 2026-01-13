pub mod client;
pub use client::*;

pub mod jwt;
pub mod response;

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::client::{Client, EnvConfig};
    use crate::response::{Currency, Payment, Status};

    use anyhow::Result;

    fn client() -> Client {
        EnvConfig::client()
    }

    fn sandbox_client() -> Client {
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
    async fn get_status() -> Result<()> {
        let mut c = client();

        let status = c.status().await?;

        assert_eq!(status.message, "OK".to_string());
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_currencies() -> Result<()> {
        let mut c = client();

        // panics if not error
        c.get_currencies().await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_full_currencies() -> Result<()> {
        let mut c = client();

        // panics if not error
        c.get_full_currencies().await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_checked_currencies() -> Result<()> {
        let mut c = client();

        // panics if not error
        c.get_checked_currencies().await.unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn get_min_payment_amount() -> Result<()> {
        let mut c = client();

        // panics if not error
        c.get_min_payment_amount(Currency::ETH, Currency::BTC)
            .await?;
        Ok(())
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
    async fn authentication() -> Result<()> {
        let mut client = client();

        // Can be ignored because credentials already parsed from env.
        let config = EnvConfig::parse();
        client
            .auth()
            .credentials()
            .email(config.email)
            .password(config.password);

        // Request a JWT against the remote API.
        client.auth().set().await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn create_payment() -> Result<()> {
        let client = client();
        client
            .payment()
            .create()
            .amount(100.0)
            .price_currency(Currency::USD)
            .pay_currency(Currency::XMR)
            .order_id("my_order_0")
            .order_description("my test order")
            .ipn_callback_url("https://test.com/")
            .post()
            .await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn get_payment() -> Result<()> {
        let mut client = client();

        let config = EnvConfig::parse();

        client.auth().set().await?;
        client.payment().state().payment_id(1).get().await?;

        Ok(())
    }
}
