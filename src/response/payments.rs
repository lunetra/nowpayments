use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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
pub struct Payment {
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
