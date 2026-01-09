# NowPayments Rust Api

[NowPayments](https://nowpayments.io/) is a Free, No ID verification Crypto2Crypto payment provider.

## Usage

```rust
// setting up client
let mut client = NPClient::new(config.api);

// creating options for your request
// deprecated
let payment = PaymentOpts::new(100, "GBP", "BTC", "http://google.com/", &id, "test order");
// new
let payment = PaymentOpts::builder()
    .price_amount(100.0)
    .price_currency(Currency::USD)
    .pay_currency(Currency::XMR")
    .order_id("my_order_0")
    .order_description("my test order")
    .ipn_callback_url("https://test.com/")
    .build();

// have to create a new JWT every 5 minutes for nowpayments
client.authenticate().await?;
let order = client.create_payment(payment).await?;

// using payment response to get status
let status = client.get_payment_status(order.payment_id).await?;
```

## Developers

- Create an `.env` based on the `.env.template`

- Increase verbosity to track bugs.

```sh
RUST_LOG="debug" && cargo test

```

## Why this fork?

S/O to the maintainer [@NikolaySch](https://github.com/NikolaiSch)
for the great code quality.

I just wanted to have more verbose errors to debug some http requests,
so I can ship this dependency to production.
This means longer compile times, but better live debugging.

## TODO

[ ]: Create error enum for when http request returns an error.
