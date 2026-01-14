use super::Client;
use crate::response::{
    payments::{RawPayment, RawPayments, Status},
    Currency, Payment,
};
use chrono::{NaiveDateTime, Utc};
use convert_case::{Case, Casing};
use std::fmt;

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
        price_currency: &Currency,
        pay_currency: &Currency,
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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum SortingField {
    #[default]
    CreatedAt,
    PaymentId,
    PaymentStatus,
    PayAddress,
    PriceAmount,
    PriceCurrency,
    PayAmount,
    PayCurrency,
    ActuallyPaid,
    OrderId,
    OrderDescription,
    PurchaseId,
    OutcomeAmount,
    OutcomeCurrency,
}
impl fmt::Display for SortingField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self);
        write!(f, "{}", name.to_case(Case::Snake))
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum OrderDirection {
    #[default]
    Desc,
    Asc,
}
impl fmt::Display for OrderDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self);
        write!(f, "{}", name.to_case(Case::Snake))
    }
}

#[bon]
impl PaymentMethods<'_> {
    #[builder(finish_fn = get)]
    #[tracing::instrument(skip_all)]
    /// Return an existing payment state.
    pub async fn all(
        &self,
        limit: u64, // 1 to 500
        page: u64,  // page count from 0 to n
        sort_by: SortingField,
        order_by: OrderDirection,
        date_from: NaiveDateTime,
        date_to: NaiveDateTime,
    ) -> Result<Vec<Payment>> {
        let client = self.client;
        if client.jwt.is_expired() {
            bail!("Expired jwt");
        }
        let from = date_from.format("%Y-%m-%d");
        let to = date_to.format("%Y-%m-%d");
        let path =
            format!("payment/?limit={limit}&page={page}&sortBy={sort_by}&orderBy={order_by}&dateFrom={from}&date_to={to}",);

        let res: String = self.client.get(&path).await?;
        let payments: RawPayments = serde_json::from_str(res.as_str())?;
        let payment: Vec<Payment> = payments.into();
        Ok(payment)
    }
}

pub struct MockMethods<'a> {
    client: &'a Client,
}
pub struct MockPaymentMethods<'a> {
    client: &'a Client,
}
impl Client {
    pub fn mock(&self) -> MockMethods<'_> {
        MockMethods { client: &self }
    }
}
impl MockMethods<'_> {
    pub fn payment(&self) -> MockPaymentMethods<'_> {
        MockPaymentMethods {
            client: self.client,
        }
    }
}
#[bon]
impl MockPaymentMethods<'_> {
    #[builder(finish_fn = post)]
    #[tracing::instrument(skip_all)]

    /// Create a payment.
    pub fn create(
        &self,
        amount: f64,
        price_currency: &Currency,
        pay_currency: &Currency,
        order_id: Option<&str>,
        order_description: Option<&str>,
    ) -> Result<Payment> {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let mut payment = Payment {
            id: 0,
            status: Status::Dummy,
            address: format!("<mock/my_{}_address>", pay_currency.network()),
            price_amount: Decimal::from_f64(amount).unwrap(),
            price_currency: price_currency.to_owned(),
            pay_amount: Decimal::from_f64(amount / 500.0).unwrap(),
            pay_currency: pay_currency.to_owned(),

            actually_paid: Some(Decimal::from_f64(amount / 1000.0).unwrap()),

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
