#![no_main]

libfuzzer_sys::fuzz_target!(|expression: &str| {
    if let Err(error) = parser::parse(expression) {
        std::hint::black_box(error.to_string());
    }
});
