use criterion::{criterion_group, criterion_main, Criterion};
use prima_datadog::{
    configuration::{Country, PrimaConfiguration},
    Datadog, TrackerConfiguration,
};

fn incr_benchmark(c: &mut Criterion) {
    // The custom action will do nothing, but does force tracking to occur
    let tracker_config = TrackerConfiguration::new().with_threshold(21).with_custom(|_, _| {});
    let configuration = PrimaConfiguration::new(
        "0.0.0.0:1234",
        "0.0.0.0:0",
        "prima_datadog_benchmarks",
        "dev".parse().unwrap(),
    )
    .with_country(Country::It)
    .with_tracker(tracker_config);
    // 20 test tags to simulate a normal to heavily tagged metric
    let tags = (0..20).map(|i| format!("tag_{}", i)).collect::<Vec<_>>();
    Datadog::init(configuration).unwrap();
    c.bench_function("incr_benchmark", |b| {
        b.iter(|| {
            // Note: clone here is ok since we only care about /relative/ perf here
            Datadog::incr("test", tags.clone());
        });
    });
}

fn incr_with_too_many_tags(c: &mut Criterion) {
    let tags = (0..22).map(|i| format!("tag_{}", i)).collect::<Vec<_>>();
    c.bench_function("incr_with_too_many_tags", |b| {
        b.iter(|| {
            Datadog::incr("test", tags.clone());
        });
    });
}

criterion_group!(benches, incr_benchmark, incr_with_too_many_tags);
criterion_main!(benches);
