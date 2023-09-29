use criterion::{criterion_group, criterion_main, Criterion};
use prima_datadog::{
    configuration::{Country, PrimaConfiguration},
    Datadog, TagTrackerConfiguration,
};

fn setup(_: &mut Criterion) {
    // The custom action will do nothing, but does force tracking to occur
    let tracker_config = TagTrackerConfiguration::new()
        .with_threshold(21)
        .with_custom_action(|_, _, _| {});
    let configuration = PrimaConfiguration::new("0.0.0.0:1234", "0.0.0.0:0", "prima_datadog_benchmarks")
        .with_country(Country::It)
        .with_tracker_configuration(tracker_config);
    Datadog::init(configuration).unwrap();
}

fn incr_benchmark(c: &mut Criterion) {
    // 20 test tags to simulate a normal to heavily tagged metric
    let tags = (0..20).map(|i| format!("tag_{}", i)).collect::<Vec<_>>();
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

criterion_group!(benches, setup, incr_benchmark, incr_with_too_many_tags);
criterion_main!(benches);
