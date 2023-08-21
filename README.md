# Prima Datadog

[![Build Status](https://drone-1.prima.it/api/badges/primait/prima_datadog.rs/status.svg)](https://drone-1.prima.it/primait/prima_datadog.rs)

This is an opinionated library to share code and approach to Datadog logging in prima.it

Refer to the [official docs](https://docs.rs/prima_datadog) for help on how to setup the library in your project

❕ Please note that `prima_datadog.rs` uses [`dogstatsd`](https://docs.rs/dogstatsd/latest/dogstatsd/), which means metrics will be sent using the **UDP** protocol, so you'll need to specify a full address with both IP and port (the default one is `8125`, but note that the library won't provide it for you). You can find more information on the [official Datadog documentation](https://docs.datadoghq.com/developers/dogstatsd/).
A full URL might then be `10.1.2.3:8125`.
