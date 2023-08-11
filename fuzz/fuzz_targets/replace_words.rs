#![no_main]
use clippit::replace_words;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        replace_words(s);
    }
});
