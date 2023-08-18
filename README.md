# Prima Datadog

[![Build Status](https://drone-1.prima.it/api/badges/primait/prima_datadog.rs/status.svg)](https://drone-1.prima.it/primait/prima_datadog.rs)

This is an opinionated library to share code and approach to Datadog logging in prima.it

Refer to the [official docs](https://docs.rs/prima_datadog) for help on how to setup the library in your project

❕ Please note that `prima_datadog.rs` uses [DogstatsD](https://docs.datadoghq.com/developers/dogstatsd/), which means metrics will be sent using the **UDP** protocol on port `8125`. You can find more information on the official Datadog documentation
