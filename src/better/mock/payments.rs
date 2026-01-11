use anyhow::Result;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bon::bon;

use crate::better::{Currency, Status};

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

    /// The effective amount of money sent to wallet
    pub outcome_amount: Option<Decimal>,
    /// The currency of the amount sent to wallet
    pub outcome_currency: Option<Currency>,

    /// Extra informations.
    /// order_id should be: <account_uuid>-<currency>
    pub order_id: String,
    pub order_description: String,

    // Date
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[bon]
impl Payment {
    #[builder]
    pub fn new(account: &Account, currency: &Currency) -> Self {
        let key = format!(
            "fake/{}_{}",
            account.uuid,
            currency.symbol().to_case(Case::Lower)
        );
        Self {
            id: 0,
            status: Status::Dummy,
            address: Uuid::new_v4().to_string(),
            price_amount: Decimal::from(5),
            price_currency: Currency::USD,
            pay_amount: Decimal::from(0),
            pay_currency: currency.to_owned(),
            order_id: key,
            order_description: String::new(),
        }
    }
}

impl Payment {
    pub async fn create_payment(account: &Account, currency: &Currency) -> Result<Self> {
        let key = format!("fake/{}_{}", account.uuid, currency.cg_id());

        let res = Payment {
            // The minimum price amount allowed by nowpayment is 2$-5$ depending on chain).
            id: 0,
            status: PaymentStatus::Unknown,
            address: "fake/<address>".to_string(),

            price_amount: Decimal::from(5),
            price_currency: Currency::USDollar,
            pay_amount: Decimal::from(0),
            pay_currency: Currency::Monero,

            order_id: key,
            order_description: "Fake Account top-up.".to_owned(),
        };
        Ok(res)
    }
}
