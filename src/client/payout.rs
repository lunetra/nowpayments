use super::Client;
use crate::response::payouts::AllPayouts;
use crate::response::payouts::Payouts;
use bon::bon;

use anyhow::Result;

pub struct PayoutMethods<'a> {
    client: &'a Client,
}
impl Client {
    pub fn payout(&self) -> PayoutMethods<'_> {
        PayoutMethods { client: &self }
    }
}
#[bon]
impl PayoutMethods<'_> {
    // TODO: finish this function.
    #[builder(finish_fn = post)]
    pub async fn create(&self) -> Result<()> {
        Ok(())
    }

    #[builder(finish_fn = get)]
    pub async fn list(&self) -> Result<AllPayouts> {
        let client = self.client;
        let req = client.get("payout").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
    #[builder(finish_fn = get)]
    pub async fn status(&self, payout_id: u64) -> Result<Payouts> {
        let client = self.client;
        let path = format!("payout/{}", payout_id.to_string());
        let res = client.get(&path).await?;

        Ok(serde_json::from_str(res.as_str())?)
    }
}
