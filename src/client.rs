use bon::bon;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

// Env vars
use dotenvy;
use std::env::var;

use crate::response;
use crate::response::conversion::SingleConversion;
use crate::response::currencies::{Currencies, FullCurrencies, SelectedCurrencies};
use crate::response::payments::EstimatedPaymentAmount;
use crate::response::payments::MinPaymentAmount;
use crate::response::payouts::AllPayouts;
use crate::response::payouts::Payouts;
use crate::response::status::Status;

// use crate::response::{currencies::Currencies, payments::Payment};
use crate::better::{currencies::Currency, payments::Payment};

use crate::{
    jwt::{JWTJson, JWT},
    response::conversion::AllConversions,
};
use anyhow::{bail, Result};
use reqwest::header;
use serde_json::Value;

static BASE_URL: &str = "https://api.nowpayments.io/v1/";
static BASE_SANDBOX_URL: &str = "https://api-sandbox.nowpayments.io/v1/";
static USERAGENT: &str = concat!("rust/nowpayments/", "0.2.3");

/// NowPayments client configuration from environment variables.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvConfig {
    pub api_key: String,
    pub sandbox_api_key: String,
    pub email: String,
    pub password: String,
}
impl EnvConfig {
    pub fn parse() -> Self {
        dotenvy::dotenv().unwrap();
        Self {
            api_key: var("NOWPAYMENTS_API_KEY").unwrap(),
            sandbox_api_key: var("NOWPAYMENTS_SANDBOX_API_KEY").unwrap(),

            email: var("NOWPAYMENTS_EMAIL").unwrap_or("null".to_owned()),
            password: var("NOWPAYMENTS_PASSWORD").unwrap_or("null".to_owned()),
        }
    }
    /// Generate a ready to use client from local environment variables.
    pub fn client() -> NPClient {
        let config = Self::parse();
        NPClient::new(config.api_key.as_str())
    }
    /// Generate a ready to use client from local environment variables.
    pub fn sandbox_client() -> NPClient {
        let config = Self::parse();
        NPClient::new_sandbox(config.api_key.as_str())
    }
}

pub struct NPClient {
    base_url: &'static str,
    email: Option<String>,
    password: Option<String>,

    jwt: JWT,
    client: reqwest::Client,
}

impl NPClient {
    pub fn new(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("x-api-key", header::HeaderValue::from_str(api_key).unwrap());

        Self {
            base_url: BASE_URL,
            client: reqwest::ClientBuilder::new()
                .user_agent(USERAGENT)
                .default_headers(headers)
                .build()
                .unwrap(),
            email: None,
            password: None,
            jwt: JWT::new(),
        }
    }

    pub fn new_sandbox(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("x-api-key", header::HeaderValue::from_str(api_key).unwrap());

        Self {
            base_url: BASE_SANDBOX_URL,
            client: reqwest::ClientBuilder::new()
                .user_agent(USERAGENT)
                .default_headers(headers)
                .build()
                .unwrap(),
            email: None,
            password: None,
            jwt: JWT::new(),
        }
    }

    pub fn set_auth(&mut self, email: String, password: String) {
        self.email = Some(email);
        self.password = Some(password);
    }

    async fn get(&self, endpoint: impl ToString) -> Result<String> {
        let endpoint = format!("{}{}", self.base_url, endpoint.to_string());

        let req = self
            .client
            .get(endpoint)
            .bearer_auth(self.jwt.get().unwrap_or("".to_string()))
            .build()?;

        let response = self.client.execute(req).await?;
        // Print headers only (no body)
        tracing::debug!("{:#?}", response);

        let body_str = response.text().await?;
        let body_json: Value = serde_json::from_str(&body_str)?;
        // Print body
        tracing::debug!("{:#?}", body_json);
        tracing::trace!("{}", serde_json::to_string_pretty(&body_json)?);

        Ok(body_str)
    }

    #[tracing::instrument(skip_all)]
    async fn post(
        &self,
        endpoint: impl Display,
        data: HashMap<&'static str, String>,
    ) -> Result<String> {
        let endpoint = format!("{}{}", self.base_url, endpoint);

        let req = self
            .client
            .post(endpoint)
            .bearer_auth(self.jwt.get().unwrap_or("".to_string()))
            .json(&data)
            .build()?;

        // Print headers only (no body)
        tracing::debug!("{:#?}", req);
        if let Some(body) = req.body() {
            let body_str = str::from_utf8(body.as_bytes().unwrap()).unwrap();
            let body_json: Value = serde_json::from_str(body_str)?;
            // Print body
            tracing::debug!("{:#?}", body_json);
            tracing::trace!("{}", serde_json::to_string_pretty(&body_json)?);
        }

        let response = self.client.execute(req).await?;
        // Print headers only (no body)
        tracing::debug!("{:#?}", response);

        let body_str = response.text().await?;
        let body_json: Value = serde_json::from_str(&body_str)?;
        // Print body
        tracing::debug!("{:#?}", body_json);
        tracing::trace!("{}", serde_json::to_string_pretty(&body_json)?);

        Ok(body_str)
    }

    // Get a JWT from the API.
    // Needed for calls that need some priviledges
    //
    // WARNING: Use only alphanumeric passwords.
    // There is an issue with reqwest and serde sanitizing json,
    // thus passwords with special chars won't work with the API.
    #[tracing::instrument(skip_all)]
    pub async fn authenticate(&mut self) -> Result<()> {
        if self.email.is_none() || self.password.is_none() {
            bail!("You did not set an email or a password.");
        }

        // Here the order matter for the later json object generation.
        // Maybe an indexmap would be better if things get more .
        let mut json = HashMap::new();
        json.insert("email", self.email.clone().unwrap());
        json.insert("password", self.password.clone().unwrap());

        let data = self.post("auth", json).await?;
        let jwt: JWTJson = serde_json::from_str(&data)?;
        tracing::trace!("Issued new jwt: {:#?}", jwt);

        self.jwt.set(jwt.token);
        Ok(())
    }
}

impl NPClient {
    pub async fn status(&self) -> Result<Status> {
        let req = self.get("status").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_currencies(&self) -> Result<Currencies> {
        let req = self.get("currencies").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_full_currencies(&self) -> Result<FullCurrencies> {
        let req = self.get("full-currencies").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_checked_currencies(&self) -> Result<SelectedCurrencies> {
        let req = self.get("merchant/coins").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
    // TODO
    pub async fn get_min_payment_amount(
        &self,
        from: impl Display,
        to: impl Display,
    ) -> Result<MinPaymentAmount> {
        let path = format!("min-amount?currency_from={}&currency_to={}", from, to);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
    // TODO
    pub async fn get_estimated_price(
        &self,
        amount: impl Display,
        from: impl Display,
        to: impl Display,
    ) -> Result<EstimatedPaymentAmount> {
        let path = format!(
            "estimate?amount={}&currency_from={}&currency_to={}",
            amount, from, to
        );
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    /// Return payment and its status
    #[tracing::instrument(skip_all)]
    pub async fn get_payment(&self, payment_id: u64) -> Result<Payment> {
        if self.jwt.is_expired() {
            bail!("Expired jwt");
        }
        let path = format!("payment/{}", payment_id);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_list_of_payments(
        &self,
        limit: impl Display,
        page: impl Display,
        sort_by: impl Display,
        order_by: impl Display,
        date_from: impl Display,
        date_to: impl Display,
    ) -> Result<Payment> {
        if self.jwt.is_expired() {
            bail!("Expired jwt");
        }
        let path = format!(
            "payment/?limit={}&page={}&sortBy={}&orderBy={}&dateFrom={}&dateTo={}",
            limit, page, sort_by, order_by, date_from, date_to
        );
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    // TODO
    pub async fn get_balance(&self) -> Result<Status> {
        let req = self.get("balance").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_payout_status(&self, payout_id: impl Display) -> Result<Payouts> {
        let path = format!("payout/{}", payout_id);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_payout_list(&self) -> Result<AllPayouts> {
        let req = self.get("payout").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_conversion_status(
        &self,
        conversion_id: impl Display,
    ) -> Result<SingleConversion> {
        let path = format!("conversion/{}", conversion_id);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_conversion_list(&self) -> Result<AllConversions> {
        let path = "conversion".to_string();
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
}

pub struct PaymentOpts {
    pub price_amount: Decimal,
    pub price_currency: Currency,
    pub pay_currency: Currency,
    pub ipn_callback_url: String,
    pub order_id: String,
    pub order_description: String,
}

#[bon]
impl PaymentOpts {
    #[builder]
    pub fn new(
        price_amount: f64,
        price_currency: Currency,
        pay_currency: Currency,
        ipn_callback_url: &str,
        order_id: &str,
        order_description: Option<&str>,
    ) -> Self {
        let order_description = match order_description {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        PaymentOpts {
            price_amount: Decimal::from_f64(price_amount).unwrap(),
            price_currency,
            pay_currency,
            ipn_callback_url: ipn_callback_url.to_string(),
            order_id: order_id.to_string(),
            order_description: order_description,
        }
    }
}

impl NPClient {
    pub async fn create_payment(&self, opts: PaymentOpts) -> Result<Payment> {
        let mut h = HashMap::new();

        h.insert("price_amount", opts.price_amount.clone().to_string());
        h.insert("price_currency", opts.price_currency.clone().to_string());
        h.insert("pay_currency", opts.pay_currency.clone().to_string());
        h.insert("ipn_callback_url", opts.ipn_callback_url.clone());
        h.insert("order_id", opts.order_id.clone());
        h.insert("order_description", opts.order_description.clone());

        let x = self.post("payment", h).await?;
        let payment: response::payments::Payment = serde_json::from_str(x.as_str())?;
        let payment: Payment = payment.into();

        Ok(payment)
    }

    pub fn get_jwt(&self) {
        dbg!(&self.jwt);
    }
}
