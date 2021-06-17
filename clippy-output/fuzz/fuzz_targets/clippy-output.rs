#![no_main]
use arbitrary::Arbitrary;
use clippy_output::ClippyOutput;
use libfuzzer_sys::fuzz_target;

#[derive(Debug, Arbitrary)]
struct Input<'a> {
    data: &'a [u8],
    width: u16,
}

fuzz_target!(|input: Input| {
    let mut output = ClippyOutput::new(input.width);
    if let Ok(s) = std::str::from_utf8(input.data) {
        output.add_str(s);
    }
    output.finish();
});
