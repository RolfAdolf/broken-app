### Шаг 4. Поиск узких мест

<img height="500" src="../broken-app/images/img.png" width="1000"/>

#### `broken_app::slow_dedup`

53% от времени выполнения `demo::main`

```rust
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {      // 751
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            out.push(*v);
            out.sort_unstable(); // 786
        }
    }
    out
}
```

#### `broken_app::slow_fib`

37% от времени выполнения `demo::main`

```rust
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fib(n - 1) + slow_fib(n - 2),     // 1250
    }
}
```

