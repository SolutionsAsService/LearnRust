//! Lesson 08: modules, a library crate, doc tests and integration tests.

/// Temperature conversions.
pub mod temperature {
    /// Celsius to Fahrenheit.
    ///
    /// ```
    /// use lesson_08_modules_testing::temperature::c_to_f;
    /// assert_eq!(c_to_f(100.0), 212.0);
    /// ```
    pub fn c_to_f(c: f64) -> f64 {
        c * 9.0 / 5.0 + 32.0
    }

    /// Fahrenheit to Celsius.
    ///
    /// ```
    /// use lesson_08_modules_testing::temperature::f_to_c;
    /// assert_eq!(f_to_c(32.0), 0.0);
    /// ```
    pub fn f_to_c(f: f64) -> f64 {
        (f - 32.0) * 5.0 / 9.0
    }

    /// Classify a Celsius reading. Kept private to the module tree on purpose.
    pub(crate) fn describe(c: f64) -> &'static str {
        match c {
            t if t < 0.0 => "freezing",
            t if t < 15.0 => "cold",
            t if t < 25.0 => "mild",
            _ => "hot",
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn round_trip() {
            for c in [-40.0, 0.0, 37.0, 100.0] {
                assert!((f_to_c(c_to_f(c)) - c).abs() < 1e-9);
            }
        }

        #[test]
        fn minus_forty_is_the_same_on_both_scales() {
            assert_eq!(c_to_f(-40.0), -40.0);
        }
    }
}

/// A one-line weather report built from the module above.
pub fn report(city: &str, celsius: f64) -> String {
    format!("{city}: {celsius:.1}°C / {:.1}°F, {}", temperature::c_to_f(celsius), temperature::describe(celsius))
}
