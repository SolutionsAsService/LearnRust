// Integration tests only see the crate's public API, like any other user would.
use lesson_08_modules_testing::{report, temperature};

#[test]
fn report_mentions_both_scales_and_a_word() {
    assert_eq!(report("Oslo", 0.0), "Oslo: 0.0°C / 32.0°F, cold");
}

#[test]
fn public_functions_are_reachable() {
    assert_eq!(temperature::f_to_c(212.0), 100.0);
}
