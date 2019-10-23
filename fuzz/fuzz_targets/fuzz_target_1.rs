#![no_main]
#[macro_use] extern crate libfuzzer_sys;

extern crate sponge_preview;

use sponge_preview::*;
fuzz_target!(|data: &[u8]| {
    let mut x = dummy::Dummy::default();
    x.absorb(data);
});
