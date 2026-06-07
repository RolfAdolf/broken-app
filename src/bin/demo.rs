use broken_app::{algo, leak_buffer, normalize, sum_even};

fn main() {
    let nums: Vec<i64> = (0..500_000).collect();
    let data: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();
    let text = " Hello World ".repeat(50_000);
    let dedup: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();

    for _ in 0..200 {
        let _ = sum_even(&nums);
        let _ = leak_buffer(&data);
        let _ = normalize(&text);
        let _ = algo::slow_fib(32);
        let _ = algo::slow_dedup(&dedup);
    }

    println!("sum_even: {}", sum_even(&nums));
    println!("non-zero bytes: {}", leak_buffer(&data));
    println!("normalize len: {}", normalize(&text).len());
    println!("fib(32): {}", algo::slow_fib(32));
    println!("dedup len: {}", algo::slow_dedup(&dedup).len());
}
