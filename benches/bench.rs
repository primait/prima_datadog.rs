use criterion::{criterion_group, criterion_main, Criterion};
use prima_datadog::{
    configuration::{Country, PrimaConfiguration},
    Datadog,
};

fn incr_benchmark(c: &mut Criterion) {
    let configuration = PrimaConfiguration::new(
        "0.0.0.0:1234",
        "0.0.0.0:0",
        "prima_datadog_benchmarks",
        "dev".parse().unwrap(),
    )
    .with_country(Country::It);
    // 20 test tags to simulate a normal to heavily tagged metric
    let tags = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
        "10".to_string(),
        "11".to_string(),
        "12".to_string(),
        "13".to_string(),
        "14".to_string(),
        "15".to_string(),
        "16".to_string(),
        "17".to_string(),
        "18".to_string(),
        "19".to_string(),
        "20".to_string(),
    ];
    Datadog::init(configuration).unwrap();
    c.bench_function("incr_benchmark", |b| {
        b.iter(|| {
            // Note: clone here is ok since we only care about /relative/ perf here
            Datadog::incr("test", tags.clone());
        });
    });
}

criterion_group!(benches, incr_benchmark);
criterion_main!(benches);
