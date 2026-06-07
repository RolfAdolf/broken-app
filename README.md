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

### Шаг 7. Проверка «после»

Источник: `artifacts/benches/before_criterion` → baseline `before`, финальный прогон → `artifacts/benches/after_criterion`.  
Входы как в `benches/criterion.rs` (large): `sum_even` 500k, `slow_fib(32)`, `slow_dedup` 5k пар, `normalize` 50k слов, `leak_buffer` 1 MiB, `average_positive` 500k.

#### Время (median)

| Функция | До | После | Ускорение | Δ |
|---------|-----|-------|-----------|---|
| `slow_dedup` | 8.48 ms | 184 µs | **×46** | −97.8% |
| `slow_fib` | 6.07 ms | 35.9 ns | **×169 000** | ~−100% |
| `sum_even` | 386 µs | 210 µs | **×1.8** | −46% |
| `average_positive` | 292 µs | 185 µs | **×1.6** | −37% |
| `normalize` | 969 µs | 2.00 ms | ×0.48 | +106% ⚠ |
| `leak_buffer` | 160 µs | 321 µs | ×0.50 | +99% ⚠ |

⚠ `normalize` / `leak_buffer`: в baseline «до» уже были частично исправленные версии; сравнение с полностью корректным кодом даёт видимую «регрессию» по времени, но не по аллокациям (см. ниже).

#### Как воспроизвести

```bash
./scripts/compare.sh                              # сохранить «до»
# … правки …
cargo bench --bench criterion -- --baseline before | tee artifacts/benches/baseline_after.txt
```

Графики: `target/criterion/report/index.html`
