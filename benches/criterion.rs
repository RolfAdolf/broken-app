use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_sum_even(c: &mut Criterion) {
    let data: Vec<i64> = (0..500_000).collect();

    c.bench_function("sum_even", |b| {
        b.iter(|| black_box(sum_even(black_box(&data))));
    });
}

fn bench_slow_fib(c: &mut Criterion) {
    c.bench_function("slow_fib", |b| {
        b.iter(|| black_box(algo::slow_fib(black_box(32))));
    });
}

fn bench_slow_dedup(c: &mut Criterion) {
    let data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();

    c.bench_function("slow_dedup", |b| {
        b.iter(|| black_box(algo::slow_dedup(black_box(&data))));
    });
}

fn bench_normalize(c: &mut Criterion) {
    let text = " Hello World ".repeat(50_000);

    c.bench_function("normalize", |b| {
        b.iter(|| black_box(normalize(black_box(text.as_str()))));
    });
}

fn bench_leak_buffer(c: &mut Criterion) {
    let data: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();

    c.bench_function("leak_buffer", |b| {
        b.iter(|| black_box(leak_buffer(black_box(data.as_slice()))));
    });
}

fn bench_average_positive(c: &mut Criterion) {
    let data: Vec<i64> = (0..500_000).map(|i| i - 250_000).collect();

    c.bench_function("average_positive", |b| {
        b.iter(|| black_box(average_positive(black_box(data.as_slice()))));
    });
}

criterion_group!(
    benches,
    bench_sum_even,
    bench_slow_fib,
    bench_slow_dedup,
    bench_normalize,
    bench_leak_buffer,
    bench_average_positive,
);
criterion_main!(benches);
