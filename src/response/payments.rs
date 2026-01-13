use super::Currency;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Status {
    #[default]
    Unknown,
    Dummy,
    Waiting,
    Confirming,
    Confirmed,
    Sending,
    PartiallyPaid,
    Finished,
    Failed,
    Refunded,
    Expired,
}
impl FromStr for Status {
    type Err = std::io::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let res = match value {
            "waiting" => Self::Waiting,
            "confirming" => Self::Confirming,
            "confirmed" => Self::Confirmed,
            "sending" => Self::Sending,
            "partially_paid" => Self::PartiallyPaid,
            "finished" => Self::Finished,
            "failed" => Self::Failed,
            "refunded" => Self::Refunded,
            "expired" => Self::Expired,
            "dummy" => Self::Dummy,
            _ => Self::Unknown,
        };
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinPaymentAmount {
    currency_from: String,
    currency_to: String,
    min_amount: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedPaymentAmount {
    currency_from: String,
    currency_to: String,
    amount_from: Decimal,
    estimated_amount: String,
}

/// Response from the /create-payment endpoint
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawPayment {
    pub payment_id: String, // must contain numbers only (u64)
    pub payment_status: String,
    pub pay_address: String,
    pub price_amount: Decimal,
    pub price_currency: String,
    pub pay_amount: Decimal,
    pub pay_currency: String,
    pub order_id: String,
    pub order_description: String,
    pub purchase_id: String,
    // Dates
    pub created_at: String,
    pub updated_at: String,
}

/// Response from the /create-payment endpoint
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawPayments {
    data: Vec<RawPayment>,
}
impl From<RawPayments> for Vec<Payment> {
    fn from(value: RawPayments) -> Self {
        let res: Vec<Payment> = value.data.iter().map(|e| e.to_owned().into()).collect();
        res
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Payment {
    pub id: u64,
    pub status: Status,
    /// Receiving address.
    pub address: String,

    /// The article price
    pub price_amount: Decimal,
    /// The article currency
    pub price_currency: Currency,
    /// The transaction amount
    pub pay_amount: Decimal,
    /// The transaction currency
    pub pay_currency: Currency,

    /// Extra informations.
    /// order_id should be: <account_uuid>-<currency>
    pub order_id: String,
    pub order_description: String,

    // Date
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<RawPayment> for Payment {
    /// Convert NowPayments Json API response to a convenient struct with methods.
    fn from(e: RawPayment) -> Self {
        Self {
            id: e.payment_id.parse().unwrap(),
            status: Status::from_str(&e.payment_status).unwrap(),
            address: e.pay_address,

            price_amount: e.price_amount,
            price_currency: Currency::from_str(&e.price_currency).unwrap(),
            pay_amount: e.pay_amount,
            pay_currency: Currency::from_str(&e.pay_currency).unwrap(),

            order_id: e.order_id,
            order_description: e.order_description,

            created_at: NaiveDateTime::parse_from_str(&e.created_at, "%Y-%m-%dT%H:%M:%S%.3fZ")
                .unwrap(),
            updated_at: NaiveDateTime::parse_from_str(&e.updated_at, "%Y-%m-%dT%H:%M:%S%.3fZ")
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use rust_decimal::prelude::FromPrimitive;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn convert() -> Result<()> {
        let api_res = RawPayment {
            payment_id: "0".to_string(),
            payment_status: "waiting".to_string(),
            pay_address: "my_fake_address".to_string(),

            price_amount: Decimal::from_f64(10.0).unwrap(),
            price_currency: "usd".to_string(),
            pay_amount: Decimal::from_f64(0.01).unwrap(),
            pay_currency: "xmr".to_string(),

            order_id: "test_id".to_string(),
            order_description: "my test".to_string(),

            purchase_id: "".to_string(),

            created_at: "2026-01-10T17:56:15.327Z".to_string(),
            updated_at: "2026-01-10T17:56:15.327Z".to_string(),
        };

        let converted_res = Payment {
            id: 0,
            status: Status::Waiting,
            address: "my_fake_address".to_string(),

            price_amount: Decimal::from_f64(10.0).unwrap(),
            price_currency: Currency::USD,
            pay_amount: Decimal::from_f64(0.01).unwrap(),
            pay_currency: Currency::XMR,

            order_id: "test_id".to_string(),
            order_description: "my test".to_string(),

            created_at: NaiveDateTime::parse_from_str(
                "2026-01-10T17:56:15.327Z",
                "%Y-%m-%dT%H:%M:%S%.3fZ",
            )
            .unwrap(),
            updated_at: NaiveDateTime::parse_from_str(
                "2026-01-10T17:56:15.327Z",
                "%Y-%m-%dT%H:%M:%S%.3fZ",
            )
            .unwrap(),
        };

        assert_eq!(converted_res, Payment::from(api_res));

        Ok(())
    }
}
