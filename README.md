# Prima Datadog

This is an opinionated library to share code and approach to Datadog logging in prima.it

#### Getting started

You need to call `Datadog::init` in your main binary, and to do so you'll need as argument a type that implements the `Configuration` trait.

Inside the `configuration` you'll find an implementation of this trait tailored for prima.it needs.

```rust
use prima_datadog::{Datadog, configuration::PrimaConfiguration};

let configuration = PrimaConfiguration::new(
    "0.0.0.0:1234", // to address
    "0.0.0.0:0", // from address
    "service_name", // namespace for all metrics
    "staging" // environment
);
Datadog::init(configuration);
```

Then you can use the macros exposed at the base level of the module

```rust
prima_datadog::incr!("test");
prima_datadog::decr!("test"; "some" => "data");
```

The first argument is the metric name. It accepts string literal (like the previous example) or a type path that implements `AsRef`

```rust
enum Metric {
    John,
    Paul,
    George,
    Ringo,
}

impl AsRef<str> for Metric {
    fn as_ref(&self) -> &str {
        match self {
            Metric::John => "john",
            Metric::Paul => "paul",
            Metric::George => "george",
            Metric::Ringo => "ringo",
        }
    }
}
```

and then

```rust
prima_datadog::incr!(Metric::John; "play" => "guitar");
prima_datadog::incr!(Metric::Paul; "play" => "bass");
prima_datadog::incr!(Metric::George; "play" => "sitar");
prima_datadog::incr!(Metric::Ringo; "play" => "drums");
```

### References

  - [Datadog docs](https://docs.datadoghq.com/getting_started/)
  - [Getting started with Datadog tags](https://docs.datadoghq.com/getting_started/tagging/)
