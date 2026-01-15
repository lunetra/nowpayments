# The NowPayments unofficial rust library.

[NowPayments](https://nowpayments.io/) is a Free, No ID verification Crypto2Crypto payment provider.

## Why this fork?

S/O to the maintainer [@NikolaySch](https://github.com/NikolaiSch)
for the great original code quality.

This fork extensively uses the
[bon](https://bon-rs.com/)
crate as a syntactic sugar for every API endpoint calls, so that we get:

- More verbose methods,
- Arguments that can be given in disorder.

Syntax example:

```rust
// Create a new payment.
client
    // The main method group.
    // can be payment, currency or auth
    .payment()
    .create() // The sub method.

    // Arguments
    .amount(100.0)
    .price_currency(&Currency::USD)
    .pay_currency(&Currency::XMR)
    .order_id("my_order_0")
    .order_description("my test order")
    .ipn_callback_url("https://test.com/")

    // Http method
    .post()
    .await?;
```

User/Developer wise this fork also added:

- More error message to improve debugging,
- improved test logging and functions tracing,
- More documentation (less jump in the source code to understand the methods).

## Usage

First set your environment variables in a `.env`
based on the `.env.template`.

```env
# Register at https://nowpayments.io/api to get your credentials.

# Generate an api key,
# for non-admin requests.
NOWPAYMENTS_SANDBOX_API_KEY=""
NOWPAYMENTS_API_KEY=""

# Your nowpayments account credentials,
# only for admin requests.
NOWPAYMENTS_EMAIL=""
# WARNING: No special chars in password.
NOWPAYMENTS_PASSWORD=""
```

Create the client with your specific credentials.

```rust
// Set up the client from your env vars.
let mut client = EnvConfig::client();
```

Then call the desired method with its arguments.

```rust
// Create a new payment.
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
```

You can mock the NowPayments API and create dummy payments
to avoid external http calls while testing.

```rust
// Create a new dummy payment.
client
  .mock()
  .payment()
  .create()
  .amount(100.0)
  .price_currency(&Currency::USD)
  .pay_currency(&Currency::XMR)
  .order_id("my_order_0")
  .order_description("my test order")
  .post()?
```

Some methods,
like seeing all your payments,
need authentication.

```rust
// NowPayments Javascript Web Tokens (JWT) expire after 5 minutes,
// so we have to create a new JWT before calling admin endpoints.
client.auth().set().await?;
// Get all youw payments.
client.payment().all().get().await?;
```

```rust
// Get payment current state.
client
    .payment()
    .state()
    .payment_id(1)
    .get()
    .await?;
```

## TODO

[ ]: http(error): cast error into an enum for each http error status.
[ ]: doc(methods): write API endpoint into each method description.
