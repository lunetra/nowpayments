use super::Client;
use crate::response::{payments::RawPayment, Currency, Payment, Status};
use chrono::{NaiveDateTime, Utc};

use bon::bon;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use std::collections::HashMap;

use anyhow::{bail, Result};
/// Just a convenience pattern so that,
/// related methods are tidy under a common namespace.
///
/// example:
///
/// ```rs
/// client().payment().create();
/// client().payment().status();
/// ```
///
pub struct PaymentMethods<'a> {
    client: &'a Client,
}
impl Client {
    pub fn payment(&self) -> PaymentMethods<'_> {
        PaymentMethods { client: &self }
    }
}
#[bon]
impl PaymentMethods<'_> {
    #[builder(finish_fn = post)]
    #[tracing::instrument(skip_all)]
    /// Create a payment.
    pub async fn create(
        &self,
        amount: f64,
        price_currency: Currency,
        pay_currency: Currency,
        ipn_callback_url: &str,
        order_id: Option<&str>,
        order_description: Option<&str>,
    ) -> Result<Payment> {
        let mut body = HashMap::from([
            (
                "price_amount",
                Decimal::from_f64(amount).unwrap().to_string(),
            ),
            ("price_currency", price_currency.to_string()),
            ("pay_currency", pay_currency.to_string()),
            ("ipn_callback_url", ipn_callback_url.to_string()),
        ]);
        if let Some(order_id) = order_id {
            body.insert("order_id", order_id.to_string());
        }
        if let Some(order_description) = order_description {
            body.insert("order_description", order_description.to_string());
        }
        let client = self.client;
        let res: String = client.post("payment", body).await?;
        let payment: RawPayment = serde_json::from_str(res.as_str())?;
        let payment: Payment = payment.into();
        Ok(payment)
    }

    #[builder(finish_fn = get)]
    #[tracing::instrument(skip_all)]
    /// Return an existing payment state.
    pub async fn state(&self, payment_id: u64) -> Result<Payment> {
        let client = self.client;
        if client.jwt.is_expired() {
            bail!("Expired jwt");
        }
        let path = format!("payment/{}", payment_id);
        let res: String = self.client.get(&path).await?;
        let payment: RawPayment = serde_json::from_str(res.as_str())?;
        let payment: Payment = payment.into();
        Ok(payment)
    }
}

pub struct MockMethods<'a> {
    payment: MockPaymentMethods<'a>,
}
pub struct MockPaymentMethods<'a> {
    client: &'a Client,
}
impl Client {
    fn mock(&self) -> MockMethods<'_> {
        MockMethods {
            payment: MockPaymentMethods { client: &self },
        }
    }
}
#[bon]
impl MockPaymentMethods<'_> {
    #[builder(finish_fn = post)]
    #[tracing::instrument(skip_all)]

    /// Create a payment.
    pub async fn create(
        &self,
        amount: f64,
        price_currency: Currency,
        pay_currency: Currency,
        order_id: Option<&str>,
        order_description: Option<&str>,
    ) -> Result<Payment> {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let mut payment = Payment {
            id: 0,
            status: Status::Unknown,
            address: format!("<mock/my_{:?}_address>", pay_currency.cg_id()),
            price_amount: Decimal::from_f64(amount).unwrap(),
            price_currency,
            pay_amount: Decimal::from_f64(amount).unwrap(),
            pay_currency,
            order_id: "".to_string(),
            order_description: "".to_string(),
            created_at: now,
            updated_at: now,
        };
        if let Some(order_id) = order_id {
            payment.order_id = order_id.to_string();
        }
        if let Some(order_description) = order_description {
            payment.order_description = order_description.to_string();
        }
        Ok(payment)
    }
}
