use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, EnumString, IntoEnumIterator};

#[derive(Serialize, Deserialize, Debug)]
pub struct Currencies {
    pub currencies: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullCurrencies {
    pub currencies: Vec<SingleCurrency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleCurrency {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub enable: bool,
    pub wallet_regex: String,
    pub priority: i64,
    pub extra_id_exists: bool,
    pub extra_id_regex: Option<String>,
    pub logo_url: String,
    pub track: bool,
    pub cg_id: String,
    pub is_maxlimit: bool,
    pub network: Option<String>,
    pub smart_contract: Option<String>,
    pub network_precision: Option<String>,
    pub explorer_link_hash: Option<String>,
    pub precision: i64,
    pub ticker: Option<String>,
    pub is_defi: bool,
    pub is_popular: bool,
    pub is_stable: bool,
    pub available_for_to_conversion: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectedCurrencies {
    #[serde(rename = "selectedCurrencies")]
    pub selected_currencies: Vec<String>,
}

/// TODO: add every currency supported
/// The enum name is the currency "code"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Currency {
    // Real money
    XMR,
    // Altcoins/Shitcoins
    SOL,
    USDCSOL,   // Stablecoin
    USDTERC20, // Stablecoin
    ETH,
    /// The DaddyCoin
    BTC,
    // Fiat
    USD,

    UNKNOWN,
}
impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Methods
impl Currency {
    /// The official currency name.
    pub fn name(&self) -> String {
        match self {
            Self::XMR => "Monero".to_string(),
            Self::BTC => "Bitcoin".to_string(),
            Self::ETH => "Ethereum".to_string(),
            Self::SOL => "Solana".to_string(),
            Self::USDCSOL => "USD Coin (Solana)".to_string(),
            Self::USDTERC20 => "Tether USD (Ethereum)".to_string(),
            Self::USD => "US Dollar".to_string(),
            _ => "Unknown".to_string(),
        }
    }
    /// The nowpayment internal currency symbol.
    pub fn cg_id(&self) -> String {
        match self {
            Self::XMR => "monero".to_string(),
            Self::BTC => "bitcoin".to_string(),
            Self::ETH => "ethereum".to_string(),
            Self::SOL => "solana".to_string(),
            Self::USDCSOL => "usdcsol".to_string(),
            Self::USDTERC20 => "tether".to_string(),
            Self::USD => "usd".to_string(),
            _ => "unknown".to_string(),
        }
    }
    /// The currency network.
    /// Used to generate the custom uri scheme.
    pub fn network(&self) -> String {
        match self {
            Self::XMR => "xmr".to_string(),
            Self::BTC => "btc".to_string(),
            Self::ETH => "eth".to_string(),
            Self::SOL => "sol".to_string(),
            Self::USDCSOL => "sol".to_string(),
            Self::USDTERC20 => "eth".to_string(),
            _ => "unknown".to_string(),
        }
    }
}

// Getters
impl Currency {
    pub fn get_stablecoins() -> Vec<Currency> {
        let items: Vec<Currency> = Self::get_all()
            .iter()
            .filter(|e| e.to_string().starts_with("USD"))
            .map(|e| e.to_owned())
            .collect();
        items
    }
    pub fn get_all() -> Vec<Currency> {
        let items: Vec<Currency> = Currency::iter()
            // Remove fiat and convenience enum variant.
            .filter(|e| !vec![Currency::USD, Currency::UNKNOWN].contains(e))
            .collect();
        items
    }
}
