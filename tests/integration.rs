use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn sum_even_empty_slice() {
    assert_eq!(sum_even(&[]), 0);
}

#[test]
fn sum_even_no_evens() {
    assert_eq!(sum_even(&[1, 3, 5]), 0);
}

#[test]
fn sum_even_includes_negative_evens() {
    assert_eq!(sum_even(&[-4, -3, 2, 7]), -2);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn leak_buffer_all_non_zero() {
    assert_eq!(leak_buffer(&[1, 2, 3]), 3);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn normalize_empty_string() {
    assert_eq!(normalize(""), "");
}

#[test]
fn normalize_collapses_multiple_spaces() {
    assert_eq!(normalize("foo   bar"), "foobar");
}

#[test]
fn normalize_splits_on_tabs_and_newlines() {
    assert_eq!(normalize("Foo\tBar\nBaz"), "foobarbaz");
}

#[test]
fn normalize_mixed_case() {
    assert_eq!(normalize("Rust LANG"), "rustlang");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    // Ожидается (5 + 15) / 2 = 10, но текущая реализация делит на все элементы.
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn average_positive_empty_slice() {
    assert!((average_positive(&[]) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn average_positive_all_negative() {
    assert!((average_positive(&[-1, -2, -3]) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn average_positive_zero_not_counted() {
    // Регрессия: старый код делил на len всего среза, включая нули.
    assert!((average_positive(&[0, 4, 6]) - 5.0).abs() < f64::EPSILON);
}
