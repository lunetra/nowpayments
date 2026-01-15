pub mod client;
pub use client::*;

pub mod jwt;
pub mod response;
pub use response::{Currency, Payment, Status};

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::client::{Client, EnvConfig};
    use crate::response::{status::ApiStatus, Currency, Payment, Status};

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
        let client = client();
        let status = client.status().await?;

        assert_eq!(status, ApiStatus::Running);
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_currencies() -> Result<()> {
        let client = client();
        // panics if not error
        client.currencies().all().get().await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_full_currencies() -> Result<()> {
        let client = client();
        // panics if not error
        client.currencies().all_with_details().get().await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_checked_currencies() -> Result<()> {
        let client = client();
        // panics if not error
        client.currencies().allowed().get().await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_min_payment_amount() -> Result<()> {
        let client = client();
        // panics if not error
        client
            .currencies()
            .min_amount()
            .from(&Currency::ETH)
            .to(&Currency::BTC)
            .get()
            .await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn get_estimate_price() -> Result<()> {
        let client = client();
        // panics if not error
        client
            .currencies()
            .price()
            .amount(2000.0)
            .from(&Currency::BTC)
            .to(&Currency::ETH)
            .get()
            .await?;
        Ok(())
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
            .password(config.password)
            .set();
        // Request a JWT against the remote API.
        client.auth().set().await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    // WARNING: Method does not work on sandbox.
    async fn mock_create_payment() -> Result<()> {
        let client = client();
        client
            .mock()
            .payment()
            .create()
            .amount(100.0)
            .price_currency(&Currency::USD)
            .pay_currency(&Currency::XMR)
            .order_id("my_order_0")
            .order_description("my test order")
            .post()?;
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
            .price_currency(&Currency::USD)
            .pay_currency(&Currency::XMR)
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
