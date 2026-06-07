pub mod algo;
pub mod concurrency;

pub fn sum_even(values: &[i64]) -> i64 {
    let mut acc = 0;
    unsafe {
        for idx in 0..=values.len()-1 {
            let v = *values.get_unchecked(idx);
            if v % 2 == 0 {
                acc += v;
            }
        }
    }
    acc
}

pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = input.len();
    let raw = Box::into_raw(boxed) as *mut u8;

    let mut count = 0;
    unsafe {
        for i in 0..len {
            if *raw.add(i) != 0_u8 {
                count += 1;
            }
        }

        drop(Box::from_raw(raw));
    }
    count
}

pub fn normalize(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<String>()
        .to_lowercase()
}

pub fn average_positive(values: &[i64]) -> f64 {
    let filtered: Vec<i64> = values.iter().copied().filter(|x| x.is_positive()).collect();
    if filtered.is_empty() {
        return 0.0;
    }
    let sum: i64 = filtered.iter().sum();
    sum as f64 / filtered.len() as f64
}

pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let raw = Box::into_raw(b);
    let val = unsafe {*raw};
    let res = val + unsafe {*raw};
    unsafe { drop(Box::from_raw(raw)) };
    res
}
