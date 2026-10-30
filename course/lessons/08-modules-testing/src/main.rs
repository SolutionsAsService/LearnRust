use lesson_08_modules_testing::report;

fn main() {
    for (city, c) in [("Oslo", -3.5), ("Lisbon", 19.0), ("Phoenix", 41.2)] {
        println!("{}", report(city, c));
    }
}
